use egui::FontDefinitions;
#[cfg(not(target_arch = "wasm32"))]
use egui_wgpu_backend::{RenderPass, ScreenDescriptor};
#[cfg(target_arch = "wasm32")]
use egui_wgpu_backend_old::{RenderPass, ScreenDescriptor};
use egui_winit_platform::{Platform, PlatformDescriptor};
use instant::Instant;
use std::fmt::{self, Debug};

#[derive(Debug)]
pub struct LoadingEguiState {
    pub load_progress: f32,
}
impl LoadingEguiState {
    pub fn new() -> Self {
        Self { load_progress: 0.0 }
    }
}

pub struct LoadingEguiPass {
    platform: Platform,
    egui_render_pass: RenderPass,
    start_time: Instant,
    size: winit::dpi::PhysicalSize<u32>,
}
impl LoadingEguiPass {
    pub fn new(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        size: winit::dpi::PhysicalSize<u32>,
    ) -> Self {
        let platform = Platform::new(PlatformDescriptor {
            physical_width: size.width,
            physical_height: size.height,
            scale_factor: 1.0,
            font_definitions: FontDefinitions::default(),
            style: Default::default(),
        });

        let egui_render_pass = RenderPass::new(&device, config.format, 1);

        Self {
            platform,
            egui_render_pass,
            start_time: Instant::now(),
            size,
        }
    }

    pub fn handle_event(&mut self, winit_event: &winit::event::Event<()>) {
        self.platform.handle_event(winit_event);
    }

    pub fn update(&mut self) {
        let elapsed_seconds = (Instant::now() - self.start_time).as_secs_f64();
        self.platform.update_time(elapsed_seconds);
    }

    pub fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        self.size = size;
    }

    pub fn render(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        view: &wgpu::TextureView,
        egui_state: &LoadingEguiState,
    ) {
        self.platform.begin_frame();

        egui::CentralPanel::default()
            .frame(egui::Frame::none().margin([100.0, 100.0]))
            .show(&self.platform.context(), |ui| {
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    ui.add(
                        egui::ProgressBar::new(egui_state.load_progress)
                            .animate(true)
                            .show_percentage(),
                    );
                    ui.label("Now Loading...");
                });
            });

        let (_output, paint_commands) = self.platform.end_frame(None);

        let paint_jobs = self.platform.context().tessellate(paint_commands);

        let screen_descriptor = ScreenDescriptor {
            physical_width: self.size.width,
            physical_height: self.size.height,
            scale_factor: 1.0,
        };
        self.egui_render_pass
            .update_texture(device, queue, &self.platform.context().font_image());
        self.egui_render_pass
            .update_buffers(device, queue, &paint_jobs, &screen_descriptor);
        self.egui_render_pass
            .execute(encoder, &view, &paint_jobs, &screen_descriptor, None)
            .unwrap();
    }
}
impl Debug for LoadingEguiPass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LoadingEguiPass")
            .field("start_time", &self.start_time)
            .field("size", &self.size)
            .finish()
    }
}
