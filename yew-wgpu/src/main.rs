mod pass;
mod wgpu_canvas;

use std::rc::Rc;

use wgpu_canvas::*;
use yew::prelude::*;

use crate::pass::TrianglePass;

struct MyRenderer {
    _instance: wgpu::Instance,
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,

    triangle_pass: pass::TrianglePass,
}
impl MyRenderer {
    async fn new(canvas_id: CanvasId) -> Self {
        let size = (300, 150);

        canvas_created(canvas_id).await;

        let instance = wgpu::Instance::new(wgpu::Backends::BROWSER_WEBGPU);
        let surface = unsafe { instance.create_surface(&CanvasWindow::new(canvas_id)) };
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
            width: size.0,
            height: size.1,
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
            triangle_pass,
        }
    }
}
impl CanvasRenderer for MyRenderer {
    fn render(&self, _delta_time: f64) {
        let output = self.surface.get_current_texture().unwrap();

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        self.triangle_pass
            .render(&mut encoder, &vek::Rgba::black(), &view);

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }
}

enum Msg {
    RendererCreated(Rc<MyRenderer>),
}

struct Model {
    renderer: Option<Rc<dyn CanvasRenderer>>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(async {
            Msg::RendererCreated(Rc::new(MyRenderer::new(CanvasId(1)).await))
        });
        Self { renderer: None }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::RendererCreated(renderer) => {
                self.renderer = Some(renderer);
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <WgpuCanvas canvas_id={CanvasId(1)} renderer={self.renderer.clone()}></WgpuCanvas>
        }
    }
}

fn main() {
    #[cfg(debug_assertions)]
    {
        wasm_logger::init(wasm_logger::Config::default());
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    }

    yew::start_app::<Model>();
}
