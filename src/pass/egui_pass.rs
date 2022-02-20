use egui::FontDefinitions;
#[cfg(not(target_arch = "wasm32"))]
use egui_wgpu_backend::{RenderPass, ScreenDescriptor};
#[cfg(target_arch = "wasm32")]
use egui_wgpu_backend_old::{RenderPass, ScreenDescriptor};
use egui_winit_platform::{Platform, PlatformDescriptor};
use instant::Instant;
use winit::window::Window;

use crate::state::*;

pub struct EguiPass {
    platform: Platform,
    egui_render_pass: RenderPass,
    start_time: Instant,
    size: winit::dpi::PhysicalSize<u32>,
}
impl EguiPass {
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
        view_state: &mut MainStateViewState,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        view: &wgpu::TextureView,
        window: &Window,
    ) {
        self.platform.begin_frame();

        match view_state.clone() {
            MainStateViewState::Triangle { label, clear_color } => {
                egui::Window::new("Test")
                    .resizable(true)
                    .scroll2([true, true])
                    .show(&self.platform.context(), |ui| {
                        egui::ComboBox::from_id_source("combo")
                            .selected_text(label)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    view_state,
                                    MainStateViewState::triangle(),
                                    "Triangle",
                                );
                                ui.selectable_value(
                                    view_state,
                                    MainStateViewState::texture(),
                                    "Texture",
                                );
                            });
                        ui.separator();
                        ui.horizontal(|ui| {
                            ui.label("Scene Clear Color");
                            let mut hsva = egui::color::Hsva::from_rgba_premultiplied(
                                clear_color.r,
                                clear_color.g,
                                clear_color.b,
                                clear_color.a,
                            );
                            egui::color_picker::color_edit_button_hsva(
                                ui,
                                &mut hsva,
                                egui::color_picker::Alpha::OnlyBlend,
                            );
                            if let MainStateViewState::Triangle { clear_color, .. } = view_state {
                                *clear_color = vek::Rgba::from(hsva.to_rgba_premultiplied());
                            }
                        });
                    });
            }
            MainStateViewState::Texture { label } => {
                egui::Window::new("Test")
                    .resizable(true)
                    .scroll2([true, true])
                    .show(&self.platform.context(), |ui| {
                        egui::ComboBox::from_id_source("combo")
                            .selected_text(label)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    view_state,
                                    MainStateViewState::triangle(),
                                    "Triangle",
                                );
                                ui.selectable_value(
                                    view_state,
                                    MainStateViewState::texture(),
                                    "Texture",
                                );
                            });
                        ui.label("Show base color texture");
                    });
            }
        }
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
