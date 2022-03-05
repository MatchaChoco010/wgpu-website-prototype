use gloo_render::request_animation_frame;
use std::{cell::RefCell, rc::Rc};
use yew::prelude::*;

use crate::*;

thread_local! {
    static CANVAS_ID: Rc<RefCell<u32>> = Rc::new(RefCell::new(1));
}

enum AppAction<App: WgpuCanvasApp + 'static> {
    Render(f64),
    StateChanged(App::Props),
    AppInitialized(Rc<RefCell<App>>, WgpuCanvasSize),
    #[cfg(web_sys_unstable_apis)]
    AppResized(WgpuCanvasSize),
}
struct AppReducer<App: WgpuCanvasApp + 'static> {
    app: Option<Rc<RefCell<App>>>,
    size: WgpuCanvasSize,
}
impl<App: WgpuCanvasApp + 'static> Default for AppReducer<App> {
    fn default() -> Self {
        Self {
            app: None,
            size: WgpuCanvasSize::default(),
        }
    }
}
impl<App: WgpuCanvasApp + 'static> Reducible for AppReducer<App> {
    type Action = AppAction<App>;
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            Self::Action::Render(delta) => {
                if let Some(app) = &self.app {
                    app.borrow_mut().update(delta, &self.size);
                }
                self
            }
            Self::Action::StateChanged(props) => {
                if let Some(app) = &self.app {
                    app.borrow_mut().update_props(&props);
                }
                self
            }
            Self::Action::AppInitialized(app, size) => Self {
                app: Some(app),
                size,
            }
            .into(),
            #[cfg(web_sys_unstable_apis)]
            Self::Action::AppResized(size) => Self {
                app: self.app.clone(),
                size,
            }
            .into(),
        }
    }
}

#[derive(Properties)]
pub struct Props<App: WgpuCanvasApp + 'static> {
    pub props: App::Props,
}
impl<App: WgpuCanvasApp + 'static> PartialEq for Props<App> {
    fn eq(&self, other: &Self) -> bool {
        self.props == other.props
    }
}

/// # Examples
///
/// ```
/// struct App;
/// impl WgpuCanvasApp for App {
///     /* snip */
/// }
///
/// # #[function_component(Test)]
/// # pub fn test() -> HTML {
/// html!{
///     <WgpuCanvas<App> />
/// }
/// # }
/// ```
#[function_component(WgpuCanvas)]
pub fn wgpu_canvas<App: WgpuCanvasApp + 'static>(props: &Props<App>) -> Html {
    // Not Supported ResizeObserver
    if !cfg!(web_sys_unstable_apis) {
        return html! {
            <div data-wgpu-canvas={"not supported"} >
                {"Unsupported because of ResizeObserver not found"}
            </div>
        };
    }

    let reducer = use_reducer(AppReducer::<App>::default);
    let canvas_ref = use_node_ref();

    // Get Canvas ID
    let id = use_state(|| {
        CANVAS_ID.with(|counter| {
            let id = *counter.borrow();
            *counter.borrow_mut() += 1;
            id
        })
    });
    let id = *id;

    // Element Size Changed
    let size = use_state(WgpuCanvasSize::default);
    #[cfg(web_sys_unstable_apis)]
    {
        use wasm_bindgen::prelude::*;
        use wasm_bindgen::JsCast;

        let canvas_ref = canvas_ref.clone();
        use_effect_with_deps(
            {
                let reducer = reducer.clone();
                let size = size.clone();
                move |canvas_ref: &NodeRef| {
                    let canvas = canvas_ref
                        .cast::<web_sys::Element>()
                        .unwrap_or_else(|| panic!("Failed to get canvas element"));
                    let f = Closure::wrap(Box::new({
                        let canvas_ref = canvas_ref.clone();
                        let size_state = size.clone();
                        move || {
                            let canvas = canvas_ref
                                .cast::<web_sys::Element>()
                                .unwrap_or_else(|| panic!("Failed to get canvas element"));
                            let rect = canvas.get_bounding_client_rect();
                            let size =
                                WgpuCanvasSize::new(rect.width() as u32, rect.height() as u32);
                            size_state.set(size);
                            reducer.dispatch(AppAction::AppResized(size))
                        }
                    }) as Box<dyn FnMut()>);
                    let observer =
                        web_sys::ResizeObserver::new(f.as_ref().unchecked_ref()).unwrap();
                    observer.observe(&canvas);
                    f.forget();
                    move || {
                        observer.disconnect();
                        size.set(WgpuCanvasSize::default());
                    }
                }
            },
            canvas_ref,
        );
    };

    // Initialize App
    let app_initialize_handle = use_async_once({
        let size = size.clone();
        || async move {
            Rc::new(RefCell::new(
                App::new(WgpuCanvasWindow::new(id, *size)).future.await,
            ))
        }
    });
    use_effect_with_deps(
        {
            let reducer = reducer.clone();
            let size = size.clone();
            move |handle: &UseAsyncOnceHandle<Rc<RefCell<App>>>| {
                if let UseAsyncOnceState::Ready(app) = handle.state() {
                    reducer.dispatch(AppAction::AppInitialized(app.clone(), *size));
                }
                || ()
            }
        },
        app_initialize_handle,
    );

    // Register Animation Callback
    let animation_handle = use_mut_ref(|| None);
    let handle = request_animation_frame({
        let reducer = reducer.clone();
        move |delta| reducer.dispatch(AppAction::Render(delta))
    });
    *animation_handle.borrow_mut() = Some(handle);

    // Update props
    use_effect_with_deps(
        move |state| {
            reducer.dispatch(AppAction::StateChanged(state.clone()));
            || ()
        },
        props.props.clone(),
    );

    html! {
        <canvas
            data-raw-handle={id.to_string()}
            width={size.width.to_string()}
            height={size.height.to_string()}
            ref={canvas_ref}/>
    }
}
