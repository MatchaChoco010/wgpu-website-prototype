use gloo_render::{request_animation_frame, AnimationFrame};
use raw_window_handle::*;
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    future::Future,
    pin::Pin,
    rc::Rc,
    task::{Poll, Waker},
};
use web_sys::HtmlCanvasElement;
use yew::prelude::*;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct CanvasId(pub u32);
pub struct CanvasWindow {
    canvas_id: CanvasId,
}
impl CanvasWindow {
    pub fn new(canvas_id: CanvasId) -> Self {
        Self { canvas_id }
    }
}
unsafe impl HasRawWindowHandle for CanvasWindow {
    fn raw_window_handle(&self) -> RawWindowHandle {
        let mut handle = WebHandle::empty();
        handle.id = self.canvas_id.0;
        RawWindowHandle::Web(handle)
    }
}

thread_local! {
    static CREATED_CANVAS_ID: RefCell<HashSet<CanvasId>> = RefCell::new(HashSet::new());
    static CREATED_CANVAS_WAKER: RefCell<HashMap<CanvasId, Waker>> = RefCell::new(HashMap::new());
}
struct CanvasCreatedFuture {
    canvas_id: CanvasId,
}
impl Future for CanvasCreatedFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        if CREATED_CANVAS_ID.with(|set| set.borrow().contains(&self.canvas_id)) {
            Poll::Ready(())
        } else {
            let waker = cx.waker().clone();
            CREATED_CANVAS_WAKER.with(|map| map.borrow_mut().insert(self.canvas_id, waker));
            Poll::Pending
        }
    }
}
fn set_canvas_created(canvas_id: CanvasId) {
    CREATED_CANVAS_ID.with(|set| set.borrow_mut().insert(canvas_id));
    CREATED_CANVAS_WAKER.with(|map| {
        if let Some(waker) = map.borrow().get(&canvas_id) {
            waker.wake_by_ref();
        }
    })
}
pub fn canvas_created(canvas_id: CanvasId) -> impl Future {
    CanvasCreatedFuture { canvas_id }
}

pub trait CanvasRenderer {
    fn render(&self, delta_time: f64);
}

#[derive(Properties)]
pub struct Props {
    pub canvas_id: CanvasId,
    pub renderer: Option<Rc<dyn CanvasRenderer>>,
}
impl PartialEq for Props {
    fn eq(&self, other: &Props) -> bool {
        self.canvas_id == other.canvas_id
            && (self.renderer.is_some() && other.renderer.is_some()
                || self.renderer.is_none() && other.renderer.is_none())
    }
}

pub enum Msg {
    Render(f64),
}

pub struct WgpuCanvas {
    node_ref: NodeRef,
    canvas: Option<HtmlCanvasElement>,
    _render_loop: Option<AnimationFrame>,
}

impl Component for WgpuCanvas {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            node_ref: NodeRef::default(),
            canvas: None,
            _render_loop: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Render(timestep) => {
                if let Some(renderer) = &ctx.props().renderer {
                    renderer.render(timestep);
                }
                let link = ctx.link().clone();
                let handle =
                    request_animation_frame(move |time| link.send_message(Msg::Render(time)));
                self._render_loop = Some(handle);
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <canvas
                ref={self.node_ref.clone()}
                data-raw-handle={ctx.props().canvas_id.0.to_string()} />
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        self.canvas = self.node_ref.cast::<HtmlCanvasElement>();

        if first_render {
            let link = ctx.link().clone();
            let handle = request_animation_frame(move |time| link.send_message(Msg::Render(time)));
            self._render_loop = Some(handle);

            set_canvas_created(ctx.props().canvas_id)
        }
    }
}
