use raw_window_handle::{HasRawWindowHandle, RawWindowHandle, WebHandle};
use std::{future::Future, pin::Pin};

#[derive(Clone, Copy)]
pub struct WgpuCanvasSize {
    pub width: u32,
    pub height: u32,
}
impl WgpuCanvasSize {
    pub(crate) fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}
impl Default for WgpuCanvasSize {
    fn default() -> Self {
        Self::new(100, 100)
    }
}

pub struct WgpuCanvasWindow {
    canvas_id: u32,
    size: WgpuCanvasSize,
}
impl WgpuCanvasWindow {
    pub(crate) fn new(canvas_id: u32, size: WgpuCanvasSize) -> Self {
        Self { canvas_id, size }
    }

    pub fn size(&self) -> &WgpuCanvasSize {
        &self.size
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
    pub(crate) future: Pin<Box<dyn Future<Output = App> + 'static>>,
}
impl<App> WgpuCanvasAppCreator<App> {
    pub fn new(creation: impl Future<Output = App> + 'static) -> Self {
        Self {
            future: Box::pin(creation),
        }
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
pub trait WgpuCanvasApp: Sized {
    type Props: PartialEq + Clone + Default;
    fn new(canvas_window: WgpuCanvasWindow) -> WgpuCanvasAppCreator<Self>;
    fn update(&mut self, delta_time: f64, size: &WgpuCanvasSize);
    fn update_props(&mut self, update: &Self::Props);
}
