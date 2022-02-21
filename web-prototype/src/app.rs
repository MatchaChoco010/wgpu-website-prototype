use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

use crate::resources::*;
use crate::state::*;

pub struct App {
    event_loop: EventLoop<()>,
    window: Window,
    resources_loader: ResourcesLoader,
}
impl App {
    pub fn new() -> Self {
        let event_loop = EventLoop::<()>::with_user_event();
        let window = WindowBuilder::new()
            .build(&event_loop)
            .unwrap_or_else(|_| panic!("Failed to create window."));
        let resources_loader = ResourcesLoader::new();

        #[cfg(target_arch = "wasm32")]
        let window = {
            // use winit::platform::web::WindowExtWebSys;
            // web_sys::window()
            //     .and_then(|window| window.document())
            //     .and_then(|document| document.body())
            //     .and_then(|body| body.append_child(&window.canvas()).ok())
            //     .unwrap_or_else(|| panic!("Failed to add canvas"));
            use wasm_bindgen::JsCast;
            use winit::platform::web::WindowBuilderExtWebSys;
            let canvas = web_sys::window()
                .and_then(|window| window.document())
                .and_then(|document| document.get_element_by_id("canvas"))
                .and_then(|canvas| canvas.dyn_into::<web_sys::HtmlCanvasElement>().ok())
                .unwrap_or_else(|| panic!("Failed to get canvas"));
            WindowBuilder::new()
                .with_canvas(Some(canvas))
                .build(&event_loop)
                .unwrap_or_else(|_| panic!("Failed to create window."))
        };

        Self {
            event_loop,
            window,
            resources_loader,
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn run(self) {
        let App {
            event_loop,
            window,
            mut resources_loader,
        } = self;

        let runtime = crate::runtime::Runtime::new();

        resources_loader.start_load(runtime.clone(), get_catalog());

        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(&window) };

        runtime.block_on(async {
            let mut state = State::new(size, instance, surface, resources_loader).await;
            event_loop.run(move |event, _, control_flow| {
                *control_flow = ControlFlow::Poll;
                state.handle_event(&event);
                match &event {
                    Event::WindowEvent {
                        event:
                            WindowEvent::CloseRequested
                            | WindowEvent::KeyboardInput {
                                input:
                                    KeyboardInput {
                                        state: ElementState::Pressed,
                                        virtual_keycode: Some(VirtualKeyCode::Escape),
                                        ..
                                    },
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    Event::WindowEvent { ref event, .. } => match event {
                        WindowEvent::Resized(physical_size) => {
                            state.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            state.resize(**new_inner_size);
                        }
                        _ => {}
                    },
                    Event::RedrawRequested(_) => {
                        state.update();
                        match state.render() {
                            Ok(_) => {}
                            Err(wgpu::SurfaceError::Lost) => state.resize(state.size()),
                            Err(wgpu::SurfaceError::OutOfMemory) => {
                                *control_flow = ControlFlow::Exit
                            }
                            Err(e) => eprintln!("{:?}", e),
                        }
                    }
                    Event::MainEventsCleared => window.request_redraw(),
                    _ => (),
                }
            });
        });
    }

    #[cfg(target_arch = "wasm32")]
    pub fn run(self) {
        use instant::Duration;

        let App {
            event_loop,
            window,
            mut resources_loader,
        } = self;

        let mut runtime = crate::runtime::Runtime::new();
        let (event_tx, event_rx) = std::sync::mpsc::channel();

        resources_loader.start_load(runtime.clone(), get_catalog());

        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(&window) };

        runtime.spawn({
            let runtime = runtime.clone();
            async move {
                let mut state = State::new(size, instance, surface, resources_loader).await;
                loop {
                    for event in event_rx.try_iter() {
                        state.handle_event(&event);
                        match &event {
                            Event::WindowEvent { ref event, .. } => match event {
                                WindowEvent::Resized(physical_size) => {
                                    state.resize(*physical_size);
                                }
                                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                                    state.resize(**new_inner_size);
                                }
                                _ => {}
                            },
                            Event::RedrawRequested(_) => {
                                state.update();
                                match state.render() {
                                    Ok(_) => {}
                                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size()),
                                    Err(wgpu::SurfaceError::OutOfMemory) => {
                                        // *control_flow = ControlFlow::Exit
                                    }
                                    Err(e) => eprintln!("{:?}", e),
                                }
                            }
                            _ => (),
                        }
                    }
                    runtime.delay(Duration::ZERO).await;
                }
            }
        });

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;
            runtime.step();
            match &event {
                Event::WindowEvent {
                    event:
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        },
                    ..
                } => *control_flow = ControlFlow::Exit,
                Event::MainEventsCleared => window.request_redraw(),
                _ => (),
            }
            event_tx.send(event.to_static().unwrap()).unwrap();
        });
    }
}
