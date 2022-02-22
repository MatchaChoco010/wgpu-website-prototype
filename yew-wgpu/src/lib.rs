use gloo_render::request_animation_frame;
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle, WebHandle};
use std::{cell::RefCell, future::Future, pin::Pin, rc::Rc};
use yew::prelude::*;

mod hooks;
use hooks::*;

pub struct WgpuCanvasWindow {
    canvas_id: u32,
}
impl WgpuCanvasWindow {
    fn new(canvas_id: u32) -> Self {
        Self { canvas_id }
    }
}
unsafe impl HasRawWindowHandle for WgpuCanvasWindow {
    fn raw_window_handle(&self) -> RawWindowHandle {
        let mut handle = WebHandle::empty();
        handle.id = self.canvas_id;
        RawWindowHandle::Web(handle)
    }
}

pub struct WgpuCanvasAppCreator<App> {
    future: Pin<Box<dyn Future<Output = App> + 'static>>,
}
impl<App> WgpuCanvasAppCreator<App> {
    pub fn new(creation: impl Future<Output = App> + 'static) -> Self {
        Self {
            future: Box::pin(creation),
        }
    }
}
pub trait WgpuCanvasApp: Sized {
    type State: PartialEq + Clone + Default;
    fn new(canvas_window: WgpuCanvasWindow) -> WgpuCanvasAppCreator<Self>;
    fn render(&self, delta_time: f64);
    fn update(&mut self, update: &Self::State);
}

enum AppAction<App: WgpuCanvasApp + 'static> {
    Render(f64),
    StateChanged(App::State),
    AppInitialized(Rc<RefCell<App>>),
}
struct AppReducer<App: WgpuCanvasApp + 'static> {
    app: Option<Rc<RefCell<App>>>,
}
impl<App: WgpuCanvasApp + 'static> Default for AppReducer<App> {
    fn default() -> Self {
        Self { app: None }
    }
}
impl<App: WgpuCanvasApp + 'static> Reducible for AppReducer<App> {
    type Action = AppAction<App>;
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            Self::Action::Render(delta) => {
                if let Some(app) = &self.app {
                    app.borrow().render(delta);
                }
                self
            }
            Self::Action::StateChanged(state) => {
                if let Some(app) = &self.app {
                    app.borrow_mut().update(&state);
                }
                self
            }
            Self::Action::AppInitialized(app) => Self { app: Some(app) }.into(),
        }
    }
}

#[derive(Properties)]
pub struct Props<App: WgpuCanvasApp + 'static> {
    pub state: App::State,
}
impl<App: WgpuCanvasApp + 'static> PartialEq for Props<App> {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state
    }
}

#[function_component(WgpuCanvas)]

pub fn wgpu_canvas<App: WgpuCanvasApp + 'static>(props: &Props<App>) -> Html {
    let reducer = use_reducer(AppReducer::<App>::default);

    let handle = use_async_once(|| async move {
        Rc::new(RefCell::new(
            App::new(WgpuCanvasWindow::new(1)).future.await,
        ))
    });

    use_effect_with_deps(
        {
            let reducer = reducer.clone();
            move |handle: &UseAsyncOnceHandle<Rc<RefCell<App>>>| {
                if let UseAsyncOnceState::Ready(app) = handle.state() {
                    reducer.dispatch(AppAction::AppInitialized(app.clone()));
                }
                || ()
            }
        },
        handle,
    );

    let animation_handle = use_mut_ref(|| None);
    let handle = request_animation_frame({
        let reducer = reducer.clone();
        move |delta| reducer.dispatch(AppAction::Render(delta))
    });
    *animation_handle.borrow_mut() = Some(handle);

    use_effect_with_deps(
        move |state| {
            reducer.dispatch(AppAction::StateChanged(state.clone()));
            || ()
        },
        props.state.clone(),
    );

    html! {
        <canvas data-raw-handle=1 width=640 height=480/>
    }
}
