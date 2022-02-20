use winit::window::Window;

use crate::pass::*;
use crate::resources::*;
use crate::state::*;

#[derive(Debug)]
pub(super) struct LoadingState {
    pub(super) surface: wgpu::Surface,
    pub(super) device: wgpu::Device,
    pub(super) queue: wgpu::Queue,
    pub(super) config: wgpu::SurfaceConfiguration,
    pub(super) size: winit::dpi::PhysicalSize<u32>,

    resources_loader: ResourcesLoader,

    loading_egui_state: LoadingEguiState,
    loading_egui_pass: LoadingEguiPass,
}
impl LoadingState {
    pub(super) async fn new(
        size: winit::dpi::PhysicalSize<u32>,
        instance: wgpu::Instance,
        surface: wgpu::Surface,
        resources_loader: ResourcesLoader,
    ) -> Self {
        // let size = window.inner_size();

        // let instance = wgpu::Instance::new(wgpu::Backends::all());
        // let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap_or_else(|| panic!("Failed to request adapter."));

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None, // Trace path
            )
            .await
            .unwrap_or_else(|_| panic!("Failed to request device."));

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8Unorm,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);

        let loading_egui_state = LoadingEguiState::new();

        let loading_egui_pass = LoadingEguiPass::new(&device, &config, size);

        Self {
            surface,
            device,
            queue,
            config,
            size,

            resources_loader,

            loading_egui_state,
            loading_egui_pass,
        }
    }
}
impl StateTrait for LoadingState {
    fn update(mut self: Box<Self>) -> Box<dyn StateTrait + Send> {
        self.loading_egui_pass.update();
        self.loading_egui_state.load_progress = self.resources_loader.progress();
        if self.resources_loader.is_loaded() {
            let resources = self
                .resources_loader
                .take_resources()
                .unwrap_or_else(|_| panic!("Failed to get resources."));
            Box::new(MainState::new(self, resources))
        } else {
            self
        }
    }

    fn handle_event(&mut self, winit_event: &winit::event::Event<()>) {
        self.loading_egui_pass.handle_event(winit_event);
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.loading_egui_pass.resize(new_size);
        }
    }

    fn size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.size
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        self.loading_egui_pass.render(
            &mut encoder,
            &self.device,
            &self.queue,
            &view,
            &self.loading_egui_state,
        );

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
