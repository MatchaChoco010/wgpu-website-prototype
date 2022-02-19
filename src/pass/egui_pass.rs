use instant::Instant;

use egui::FontDefinitions;
#[cfg(not(target_arch = "wasm32"))]
use egui_wgpu_backend::{RenderPass, ScreenDescriptor};
#[cfg(target_arch = "wasm32")]
use egui_wgpu_backend_old::{RenderPass, ScreenDescriptor};
use egui_winit_platform::{Platform, PlatformDescriptor};
use winit::window::Window;

pub struct EguiPass {
    platform: Platform,
    egui_render_pass: RenderPass,
    start_time: Instant,
    size: winit::dpi::PhysicalSize<u32>,
}
impl EguiPass {
    pub fn new(
        window: &Window,
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        size: winit::dpi::PhysicalSize<u32>,
    ) -> Self {
        let platform = Platform::new(PlatformDescriptor {
            physical_width: size.width,
            physical_height: size.height,
            scale_factor: window.scale_factor(),
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
        egui_state: &mut crate::egui_state::EguiState,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        view: &wgpu::TextureView,
        window: &Window,
    ) {
        self.platform.begin_frame();
        egui::Window::new("My Window")
            .resizable(true)
            .scroll2([true, true])
            .show(&self.platform.context(), |ui| {
                ui.heading("Hello");
                ui.label("Hello egui!");
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Scene Clear Color");
                    let mut hsva = egui::color::Hsva::from_rgba_premultiplied(
                        egui_state.clear_color.r,
                        egui_state.clear_color.g,
                        egui_state.clear_color.b,
                        egui_state.clear_color.a,
                    );
                    egui::color_picker::color_edit_button_hsva(
                        ui,
                        &mut hsva,
                        egui::color_picker::Alpha::OnlyBlend,
                    );
                    egui_state.clear_color = vek::Rgba::from(hsva.to_rgba_premultiplied());
                });
            });
        let (_output, paint_commands) = self.platform.end_frame(Some(window));

        let paint_jobs = self.platform.context().tessellate(paint_commands);

        let screen_descriptor = ScreenDescriptor {
            physical_width: self.size.width,
            physical_height: self.size.height,
            scale_factor: window.scale_factor() as f32,
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
