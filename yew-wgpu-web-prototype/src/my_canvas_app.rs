use yew_wgpu::*;

use crate::pass::TrianglePass;

#[derive(Clone, PartialEq)]
pub struct MyCanvasAppProps {
    pub clear_color: vek::Rgba<f32>,
}
impl Default for MyCanvasAppProps {
    fn default() -> Self {
        Self {
            clear_color: vek::Rgba::<f32>::black(),
        }
    }
}

pub struct MyCanvasApp {
    _instance: wgpu::Instance,
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: WgpuCanvasSize,

    triangle_pass: TrianglePass,
}
impl WgpuCanvasApp for MyCanvasApp {
    type Props = MyCanvasAppProps;

    fn new(canvas_window: WgpuCanvasWindow) -> WgpuCanvasAppCreator<Self> {
        WgpuCanvasAppCreator::new(async move {
            let size = *canvas_window.size();

            let instance = wgpu::Instance::new(wgpu::Backends::BROWSER_WEBGPU);
            let surface = unsafe { instance.create_surface(&canvas_window) };
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

            let triangle_pass = TrianglePass::new(&device, &config);

            Self {
                _instance: instance,
                surface,
                device,
                queue,
                config,
                size,
                triangle_pass,
            }
        })
    }

    fn update(&mut self, _delta_time: f64, size: &WgpuCanvasSize) {
        if size.width > 0 && size.height > 0 {
            self.size = *size;
            self.config.width = size.width;
            self.config.height = size.height;
            self.surface.configure(&self.device, &self.config);
        }

        let output = self.surface.get_current_texture().unwrap();

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        self.triangle_pass.render(&mut encoder, &view);

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }

    fn update_props(&mut self, props: &Self::Props) {
        self.triangle_pass.set_clear_color(props.clear_color);
    }
}
