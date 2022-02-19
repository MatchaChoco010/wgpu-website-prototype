use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

mod app;

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

    pub async fn run(self) {
        let App { event_loop, window } = self;

        let mut app = app::App::new(&window).await;

        event_loop.run(move |event, _, control_flow| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {
                WindowEvent::Resized(physical_size) => {
                    app.resize(*physical_size);
                }
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    app.resize(**new_inner_size);
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
                app.update();
                match app.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => app.resize(app.size),
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                window.request_redraw();
            }
            _ => {}
        });
    }
}
