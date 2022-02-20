use crate::resources::*;
use crate::state::*;

pub(super) trait StateTrait: std::fmt::Debug {
    fn update(self: Box<Self>) -> Box<dyn StateTrait + Send>;
    fn handle_event(&mut self, winit_event: &winit::event::Event<()>);
    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>);
    fn size(&self) -> winit::dpi::PhysicalSize<u32>;
    fn render(&mut self) -> Result<(), wgpu::SurfaceError>;
}

#[derive(Debug)]
pub struct State {
    state: Option<Box<dyn StateTrait + Send>>,
}
impl State {
    pub async fn new(
        size: winit::dpi::PhysicalSize<u32>,
        instance: wgpu::Instance,
        surface: wgpu::Surface,
        resources_loader: ResourcesLoader,
    ) -> Self {
        Self {
            state: Some(Box::new(
                LoadingState::new(size, instance, surface, resources_loader).await,
            )),
        }
    }

    pub fn update(&mut self) {
        self.state = Some(self.state.take().unwrap().update());
    }

    pub fn handle_event(&mut self, winit_event: &winit::event::Event<()>) {
        self.state.as_mut().unwrap().handle_event(winit_event)
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.state.as_mut().unwrap().resize(new_size)
    }

    pub fn size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.state.as_ref().unwrap().size()
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.state.as_mut().unwrap().render()
    }
}
