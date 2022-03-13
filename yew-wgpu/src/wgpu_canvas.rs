use derivative::*;
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

// #[derive(Properties)]
#[derive(Properties, Derivative)]
#[derivative(PartialEq, Clone)]
pub struct Props<App: WgpuCanvasApp + 'static> {
    #[prop_or(true)]
    pub animated: bool,

    #[prop_or_default]
    pub onabort: Callback<Event>,
    #[prop_or_default]
    pub onauxclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub onblur: Callback<FocusEvent>,
    #[prop_or_default]
    pub oncancel: Callback<Event>,
    #[prop_or_default]
    pub oncanplay: Callback<Event>,
    #[prop_or_default]
    pub oncanplaythrough: Callback<Event>,
    #[prop_or_default]
    pub onchange: Callback<Event>,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub onclose: Callback<Event>,
    #[prop_or_default]
    pub oncontextmenu: Callback<MouseEvent>,
    #[prop_or_default]
    pub oncuechange: Callback<Event>,
    #[prop_or_default]
    pub ondblclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub ondrag: Callback<DragEvent>,
    #[prop_or_default]
    pub ondragend: Callback<DragEvent>,
    #[prop_or_default]
    pub ondragenter: Callback<DragEvent>,
    #[prop_or_default]
    pub ondragexit: Callback<DragEvent>,
    #[prop_or_default]
    pub ondragleave: Callback<DragEvent>,
    #[prop_or_default]
    pub ondragover: Callback<DragEvent>,
    #[prop_or_default]
    pub ondragstart: Callback<DragEvent>,
    #[prop_or_default]
    pub ondrop: Callback<DragEvent>,
    #[prop_or_default]
    pub ondurationchange: Callback<Event>,
    #[prop_or_default]
    pub onemptied: Callback<Event>,
    #[prop_or_default]
    pub onended: Callback<Event>,
    #[prop_or_default]
    pub onerror: Callback<Event>,
    #[prop_or_default]
    pub onfocus: Callback<FocusEvent>,
    #[prop_or_default]
    pub onfocusin: Callback<FocusEvent>,
    #[prop_or_default]
    pub onfocusout: Callback<FocusEvent>,
    #[prop_or_default]
    pub onformdata: Callback<Event>,
    #[prop_or_default]
    pub oninput: Callback<InputEvent>,
    #[prop_or_default]
    pub oninvalid: Callback<Event>,
    #[prop_or_default]
    pub onkeydown: Callback<KeyboardEvent>,
    #[prop_or_default]
    pub onkeypress: Callback<KeyboardEvent>,
    #[prop_or_default]
    pub onkeyup: Callback<KeyboardEvent>,
    #[prop_or_default]
    pub onload: Callback<Event>,
    #[prop_or_default]
    pub onloadeddata: Callback<Event>,
    #[prop_or_default]
    pub onloadedmetadata: Callback<Event>,
    #[prop_or_default]
    pub onloadstart: Callback<ProgressEvent>,
    #[prop_or_default]
    pub onmousedown: Callback<MouseEvent>,
    #[prop_or_default]
    pub onmouseenter: Callback<MouseEvent>,
    #[prop_or_default]
    pub onmouseleave: Callback<MouseEvent>,
    #[prop_or_default]
    pub onmousemove: Callback<MouseEvent>,
    #[prop_or_default]
    pub onmouseout: Callback<MouseEvent>,
    #[prop_or_default]
    pub onmouseover: Callback<MouseEvent>,
    #[prop_or_default]
    pub onmouseup: Callback<MouseEvent>,
    #[prop_or_default]
    pub onpause: Callback<Event>,
    #[prop_or_default]
    pub onplay: Callback<Event>,
    #[prop_or_default]
    pub onplaying: Callback<Event>,
    #[prop_or_default]
    pub onprogress: Callback<ProgressEvent>,
    #[prop_or_default]
    pub onratechange: Callback<Event>,
    #[prop_or_default]
    pub onreset: Callback<Event>,
    #[prop_or_default]
    pub onresize: Callback<Event>,
    #[prop_or_default]
    pub onscroll: Callback<Event>,
    #[prop_or_default]
    pub onsecuritypolicyviolation: Callback<Event>,
    #[prop_or_default]
    pub onseeked: Callback<Event>,
    #[prop_or_default]
    pub onseeking: Callback<Event>,
    #[prop_or_default]
    pub onselect: Callback<Event>,
    #[prop_or_default]
    pub onslotchange: Callback<Event>,
    #[prop_or_default]
    pub onstalled: Callback<Event>,
    #[prop_or_default]
    pub onsubmit: Callback<FocusEvent>,
    #[prop_or_default]
    pub onsuspend: Callback<Event>,
    #[prop_or_default]
    pub ontimeupdate: Callback<Event>,
    #[prop_or_default]
    pub ontoggle: Callback<Event>,
    #[prop_or_default]
    pub onvolumechange: Callback<Event>,
    #[prop_or_default]
    pub onwaiting: Callback<Event>,
    #[prop_or_default]
    pub onwheel: Callback<WheelEvent>,
    #[prop_or_default]
    pub oncopy: Callback<Event>,
    #[prop_or_default]
    pub oncut: Callback<Event>,
    #[prop_or_default]
    pub onpaste: Callback<Event>,
    #[prop_or_default]
    pub onanimationcancel: Callback<AnimationEvent>,
    #[prop_or_default]
    pub onanimationend: Callback<AnimationEvent>,
    #[prop_or_default]
    pub onanimationiteration: Callback<AnimationEvent>,
    #[prop_or_default]
    pub onanimationstart: Callback<AnimationEvent>,
    #[prop_or_default]
    pub ongotpointercapture: Callback<PointerEvent>,
    #[prop_or_default]
    pub onloadend: Callback<ProgressEvent>,
    #[prop_or_default]
    pub onlostpointercapture: Callback<PointerEvent>,
    #[prop_or_default]
    pub onpointercancel: Callback<PointerEvent>,
    #[prop_or_default]
    pub onpointerdown: Callback<PointerEvent>,
    #[prop_or_default]
    pub onpointerenter: Callback<PointerEvent>,
    #[prop_or_default]
    pub onpointerleave: Callback<PointerEvent>,
    #[prop_or_default]
    pub onpointerlockchange: Callback<Event>,
    #[prop_or_default]
    pub onpointerlockerror: Callback<Event>,
    #[prop_or_default]
    pub onpointermove: Callback<PointerEvent>,
    #[prop_or_default]
    pub onpointerout: Callback<PointerEvent>,
    #[prop_or_default]
    pub onpointerover: Callback<PointerEvent>,
    #[prop_or_default]
    pub onpointerup: Callback<PointerEvent>,
    #[prop_or_default]
    pub onselectionchange: Callback<Event>,
    #[prop_or_default]
    pub onselectstart: Callback<Event>,
    #[prop_or_default]
    pub onshow: Callback<Event>,
    #[prop_or_default]
    pub ontouchcancel: Callback<TouchEvent>,
    #[prop_or_default]
    pub ontouchend: Callback<TouchEvent>,
    #[prop_or_default]
    pub ontouchmove: Callback<TouchEvent>,
    #[prop_or_default]
    pub ontouchstart: Callback<TouchEvent>,
    #[prop_or_default]
    pub ontransitioncancel: Callback<TransitionEvent>,
    #[prop_or_default]
    pub ontransitionend: Callback<TransitionEvent>,
    #[prop_or_default]
    pub ontransitionrun: Callback<TransitionEvent>,
    #[prop_or_default]
    pub ontransitionstart: Callback<TransitionEvent>,

    pub props: App::Props,
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

    let animated = props.animated;
    let Props::<App> {
        props,

        onabort,
        onauxclick,
        onblur,
        oncancel,
        oncanplay,
        oncanplaythrough,
        onchange,
        onclick,
        onclose,
        oncontextmenu,
        oncuechange,
        ondblclick,
        ondrag,
        ondragend,
        ondragenter,
        ondragexit,
        ondragleave,
        ondragover,
        ondragstart,
        ondrop,
        ondurationchange,
        onemptied,
        onended,
        onerror,
        onfocus,
        onfocusin,
        onfocusout,
        onformdata,
        oninput,
        oninvalid,
        onkeydown,
        onkeypress,
        onkeyup,
        onload,
        onloadeddata,
        onloadedmetadata,
        onloadstart,
        onmousedown,
        onmouseenter,
        onmouseleave,
        onmousemove,
        onmouseout,
        onmouseover,
        onmouseup,
        onpause,
        onplay,
        onplaying,
        onprogress,
        onratechange,
        onreset,
        onresize,
        onscroll,
        onsecuritypolicyviolation,
        onseeked,
        onseeking,
        onselect,
        onslotchange,
        onstalled,
        onsubmit,
        onsuspend,
        ontimeupdate,
        ontoggle,
        onvolumechange,
        onwaiting,
        onwheel,
        oncopy,
        oncut,
        onpaste,
        onanimationcancel,
        onanimationend,
        onanimationiteration,
        onanimationstart,
        ongotpointercapture,
        onloadend,
        onlostpointercapture,
        onpointercancel,
        onpointerdown,
        onpointerenter,
        onpointerleave,
        onpointerlockchange,
        onpointerlockerror,
        onpointermove,
        onpointerout,
        onpointerover,
        onpointerup,
        onselectionchange,
        onselectstart,
        onshow,
        ontouchcancel,
        ontouchend,
        ontouchmove,
        ontouchstart,
        ontransitioncancel,
        ontransitionend,
        ontransitionrun,
        ontransitionstart,
        ..
    } = props;

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
    #[cfg(web_sys_unstable_apis)]
    {
        use wasm_bindgen::prelude::*;
        use wasm_bindgen::JsCast;

        let canvas_ref = canvas_ref.clone();
        {
            let reducer = reducer.clone();
            let animated = animated;
            use_effect_with_deps(
                move |canvas_ref: &NodeRef| {
                    let canvas = canvas_ref
                        .cast::<web_sys::Element>()
                        .unwrap_or_else(|| panic!("Failed to get canvas element"));
                    let f = Closure::wrap(Box::new({
                        let canvas_ref = canvas_ref.clone();
                        move || {
                            let canvas = canvas_ref
                                .cast::<web_sys::Element>()
                                .unwrap_or_else(|| panic!("Failed to get canvas element"));
                            let rect = canvas.get_bounding_client_rect();
                            let size =
                                WgpuCanvasSize::new(rect.width() as u32, rect.height() as u32);
                            reducer.dispatch(AppAction::AppResized(size));
                            if !animated {
                                reducer.dispatch(AppAction::Render(0.0));
                            }
                        }
                    }) as Box<dyn FnMut()>);
                    let observer =
                        web_sys::ResizeObserver::new(f.as_ref().unchecked_ref()).unwrap();
                    observer.observe(&canvas);
                    f.forget();
                    move || observer.disconnect()
                },
                canvas_ref,
            );
        };
    }

    // Initialize App
    {
        let app_initialize_handle = {
            let reducer = reducer.clone();
            use_async_once({
                || async move {
                    Rc::new(RefCell::new(
                        App::new(WgpuCanvasWindow::new(id, reducer.size))
                            .future
                            .await,
                    ))
                }
            })
        };
        {
            let reducer = reducer.clone();
            let props = props.clone();
            use_effect_with_deps(
                move |handle: &UseAsyncOnceHandle<Rc<RefCell<App>>>| {
                    if let UseAsyncOnceState::Ready(app) = handle.state() {
                        reducer.dispatch(AppAction::AppInitialized(app.clone(), reducer.size));
                        reducer.dispatch(AppAction::StateChanged(props));
                        reducer.dispatch(AppAction::Render(0.0));
                    }
                    || ()
                },
                app_initialize_handle,
            );
        }
    }

    // Register Animation Callback
    {
        let reducer = reducer.clone();
        let animated = animated;
        use_effect(move || {
            let mut handle = if animated {
                Some(request_animation_frame(move |delta| {
                    reducer.dispatch(AppAction::Render(delta))
                }))
            } else {
                None
            };
            move || {
                if let Some(handle) = handle.take() {
                    drop(handle)
                }
            }
        });
    }

    // Update props
    {
        let reducer = reducer.clone();
        use_effect_with_deps(
            move |(animated, props)| {
                reducer.dispatch(AppAction::StateChanged(props.clone()));
                if !animated {
                    reducer.dispatch(AppAction::Render(0.0));
                }
                || ()
            },
            (animated, props.clone()),
        );
    }

    html! {
        <canvas
            data-raw-handle={id.to_string()}
            width={reducer.size.width.to_string()}
            height={reducer.size.height.to_string()}
            ref={canvas_ref}

            {onabort}
            {onauxclick}
            {onblur}
            {oncancel}
            {oncanplay}
            {oncanplaythrough}
            {onchange}
            {onclick}
            {onclose}
            {oncontextmenu}
            {oncuechange}
            {ondblclick}
            {ondrag}
            {ondragend}
            {ondragenter}
            {ondragexit}
            {ondragleave}
            {ondragover}
            {ondragstart}
            {ondrop}
            {ondurationchange}
            {onemptied}
            {onended}
            {onerror}
            {onfocus}
            {onfocusin}
            {onfocusout}
            {onformdata}
            {oninput}
            {oninvalid}
            {onkeydown}
            {onkeypress}
            {onkeyup}
            {onload}
            {onloadeddata}
            {onloadedmetadata}
            {onloadstart}
            {onmousedown}
            {onmouseenter}
            {onmouseleave}
            {onmousemove}
            {onmouseout}
            {onmouseover}
            {onmouseup}
            {onpause}
            {onplay}
            {onplaying}
            {onprogress}
            {onratechange}
            {onreset}
            {onresize}
            {onscroll}
            {onsecuritypolicyviolation}
            {onseeked}
            {onseeking}
            {onselect}
            {onslotchange}
            {onstalled}
            {onsubmit}
            {onsuspend}
            {ontimeupdate}
            {ontoggle}
            {onvolumechange}
            {onwaiting}
            {onwheel}
            {oncopy}
            {oncut}
            {onpaste}
            {onanimationcancel}
            {onanimationend}
            {onanimationiteration}
            {onanimationstart}
            {ongotpointercapture}
            {onloadend}
            {onlostpointercapture}
            {onpointercancel}
            {onpointerdown}
            {onpointerenter}
            {onpointerleave}
            {onpointerlockchange}
            {onpointerlockerror}
            {onpointermove}
            {onpointerout}
            {onpointerover}
            {onpointerup}
            {onselectionchange}
            {onselectstart}
            {onshow}
            {ontouchcancel}
            {ontouchend}
            {ontouchmove}
            {ontouchstart}
            {ontransitioncancel}
            {ontransitionend}
            {ontransitionrun}
            {ontransitionstart}
            />
    }
}
