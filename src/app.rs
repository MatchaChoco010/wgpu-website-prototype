use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,

    egui_state: crate::egui_state::EguiState,

    triangle_pass: crate::pass::TrianglePass,
    egui_pass: crate::pass::EguiPass,
}
impl State {
    async fn new(window: &Window) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
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

        let egui_state = crate::egui_state::EguiState::new();

        let triangle_pass = crate::pass::TrianglePass::new(&device, &config);
        let egui_pass = crate::pass::EguiPass::new(window, &device, &config, size);

        Self {
            surface,
            device,
            queue,
            config,
            size,

            egui_state,

            triangle_pass,
            egui_pass,
        }
    }

    fn handle_event(&mut self, winit_event: &winit::event::Event<()>) {
        self.egui_pass.handle_event(winit_event);
    }

    fn update(&mut self) {
        self.egui_pass.update();
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }

        self.egui_pass.resize(new_size);
    }

    fn render(&mut self, window: &Window) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        self.triangle_pass
            .render(&mut encoder, &self.egui_state, &view);
        self.egui_pass.render(
            &mut encoder,
            &mut self.egui_state,
            &self.device,
            &self.queue,
            &view,
            window,
        );

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

pub struct App {
    event_loop: EventLoop<()>,
    window: Window,
}
impl App {
    pub fn new() -> Self {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .build(&event_loop)
            .unwrap_or_else(|_| panic!("Failed to create window."));

        #[cfg(target_arch = "wasm32")]
        {
            use winit::platform::web::WindowExtWebSys;
            web_sys::window()
                .and_then(|window| window.document())
                .and_then(|document| document.body())
                .and_then(|body| body.append_child(&window.canvas()).ok())
                .unwrap_or_else(|| panic!("Failed to add canvas"));
        }

        Self { event_loop, window }
    }

    async fn start(self) {
        let App { event_loop, window } = self;

        let mut state = State::new(&window).await;
        event_loop.run(move |event, _, control_flow| {
            state.handle_event(&event);
            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == window.id() => match event {
                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        state.resize(**new_inner_size);
                    }
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    _ => {}
                },
                Event::RedrawRequested(window_id) if window_id == window.id() => {
                    state.update();
                    match state.render(&window) {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                        Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                        Err(e) => eprintln!("{:?}", e),
                    }
                }
                Event::MainEventsCleared => window.request_redraw(),
                _ => {}
            }
        });
    }

    pub fn run(self) {
        #[cfg(target_arch = "wasm32")]
        wasm_bindgen_futures::spawn_local(self.start());

        #[cfg(not(target_arch = "wasm32"))]
        pollster::block_on(self.start());
    }
}
