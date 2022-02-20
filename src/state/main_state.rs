use crate::pass::*;
use crate::resources::*;
use crate::state::*;

#[derive(Debug, Clone, PartialEq)]
pub enum MainStateViewState {
    Triangle {
        label: String,
        clear_color: vek::Rgba<f32>,
    },
    Texture {
        label: String,
    },
}
impl MainStateViewState {
    pub fn new() -> Self {
        Self::triangle()
    }

    pub fn triangle() -> Self {
        Self::Triangle {
            label: "Triangle".into(),
            clear_color: vek::Rgba {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            },
        }
    }

    pub fn texture() -> Self {
        Self::Texture {
            label: "Texture".into(),
        }
    }
}

#[derive(Debug)]
pub(super) struct MainState {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,

    resources: Resources,

    view_state: MainStateViewState,

    triangle_pass: TrianglePass,
    texture_pass: TexturePass,

    egui_pass: EguiPass,
}
impl MainState {
    pub(super) fn new(loading_state: Box<LoadingState>, resources: Resources) -> Self {
        let LoadingState {
            surface,
            device,
            queue,
            config,
            size,
            ..
        } = *loading_state;

        let view_state = MainStateViewState::new();

        let triangle_pass = TrianglePass::new(&device, &config);
        let texture_pass = TexturePass::new(&device);

        let egui_pass = EguiPass::new(&device, &config, size);

        Self {
            surface,
            device,
            queue,
            config,
            size,

            resources,

            view_state,

            triangle_pass,
            texture_pass,

            egui_pass,
        }
    }
}
impl StateTrait for MainState {
    fn update(mut self: Box<Self>) -> Box<dyn StateTrait + Send> {
        self.egui_pass.update();
        self
    }

    fn handle_event(&mut self, winit_event: &winit::event::Event<()>) {
        self.egui_pass.handle_event(winit_event);
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.egui_pass.resize(new_size);
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

        match &self.view_state {
            MainStateViewState::Triangle { clear_color, .. } => {
                self.triangle_pass.render(&mut encoder, clear_color, &view);
            }
            &MainStateViewState::Texture { .. } => {
                self.texture_pass.render(
                    &mut encoder,
                    &self.device,
                    &self.queue,
                    &self.config,
                    &view,
                    &self.resources,
                );
            }
        }

        self.egui_pass.render(
            &mut encoder,
            &mut self.view_state,
            &self.device,
            &self.queue,
            &view,
        );

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
