use wgpu_multiplatform::*;
use winit::event::*;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

async fn run(
    event_loop: EventLoop<()>,
    window: Window,
    sc_format: wgpu::TextureFormat,
) {
    let mut demo: Demo = Demo::new(&window, sc_format).await.unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::WindowEvent {
                event: w_event,
                ..
            } => {
                match w_event {
                    WindowEvent::Resized(size) => {
                        demo.resize(size.width, size.height);
                    }
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    }
                    WindowEvent::KeyboardInput {
                        input: KeyboardInput {
                            state,
                            virtual_keycode: Some(key_code),
                            ..
                        },
                        ..
                    } => {
                        match (key_code, state == ElementState::Pressed) {
                            (VirtualKeyCode::Escape, true) => {
                                *control_flow = ControlFlow::Exit;
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            Event::RedrawRequested(_) => {
                demo.render();
            }
            _ => {}
        }
    })
}

fn main() {
    // env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("WebGPU on Nuxt")
        .build(&event_loop)
        .unwrap();

    #[cfg(not(target_arch = "wasm32"))]
    {
        subscriber::initialize_default_subscriber(None);
        futures::executor::block_on(run(event_loop, window, wgpu::TextureFormat::Bgra8UnormSrgb));
    }

    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init().expect("Could not initialize logger");
        
        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| doc.body())
            .and_then(|body| {
                body.append_child(&web_sys::Element::from(window.canvas())).ok()
            })
            .expect("Could not append canvas to document body");
        wasm_bindgen_futures::spawn_local(run(event_loop, window, wgpu::TextureFormat::Bgra8Unorm));
    }
}