use anyhow::*;
use wasm_bindgen::prelude::*;
use winit::event::*;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

pub struct Demo {
    surface: wgpu::Surface,
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    device: wgpu::Device,
    queue: wgpu::Queue,
    pipeline: wgpu::RenderPipeline,
}

impl Demo {
    pub async fn new(window: &Window, swap_chain_format: wgpu::TextureFormat) -> Result<Self> {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
            })
            .await
            .context("Failed to find a valid adapter!")?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    // features: wgpu::Features::PUSH_CONSTANTS,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits {
                        max_push_constant_size: 4, // size of f32
                        ..wgpu::Limits::default()
                    },
                    shader_validation: true,
                },
                None,
            )
            .await
            .context("Failed to create device!")?;

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: swap_chain_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Mailbox,
        };
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        let vs_module = device.create_shader_module(wgpu::include_spirv!("shader.vert.spv"));
        let fs_module = device.create_shader_module(wgpu::include_spirv!("shader.frag.spv"));

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Main Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[
                // wgpu::PushConstantRange {
                //     stages: wgpu::ShaderStage::FRAGMENT,
                //     range: 0..4,
                // },
            ],
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex_stage: wgpu::ProgrammableStageDescriptor {
                module: &vs_module,
                entry_point: "main",
            },
            fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                module: &fs_module,
                entry_point: "main",
            }),
            rasterization_state: Some(Default::default()),
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
            color_states: &[sc_desc.format.into()],
            depth_stencil_state: None,
            vertex_state: wgpu::VertexStateDescriptor {
                index_format: wgpu::IndexFormat::Uint16,
                vertex_buffers: &[],
            },
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        });

        Ok(Self {
            surface,
            sc_desc,
            swap_chain,
            device,
            queue,
            pipeline,
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.sc_desc.width = width;
        self.sc_desc.height = height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }

    pub fn render(&mut self) {
        match self.swap_chain.get_current_frame() {
            Ok(frame) => {
                let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });
                {
                    let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        color_attachments: &[
                            wgpu::RenderPassColorAttachmentDescriptor {
                                attachment: &frame.output.view,
                                resolve_target: None,
                                ops: wgpu::Operations::default(),
                            }
                        ],
                        depth_stencil_attachment: None,
                    });
                    pass.set_pipeline(&self.pipeline);
                    pass.draw(0..3, 0..1);
                }
                self.queue.submit(Some(encoder.finish()));
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }
}

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

#[wasm_bindgen]
pub fn demo() {
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
            .expect("Could not find window!")
            .document()
            .expect("Could not find document for window!")
            .get_element_by_id("wgpu-display")
            .expect("Could not find element #wgpu-display!")
            .append_child(&web_sys::Element::from(window.canvas()))
            .expect("Could not append canvas to #wgpu-display!");
            // .and_then(|win| win.document())
            // .and_then(|doc| doc.body())
            // .and_then(|body| {
            //     body.append_child(&web_sys::Element::from(window.canvas())).ok()
            // })
            // .expect("Could not append canvas to document body");
        wasm_bindgen_futures::spawn_local(run(event_loop, window, wgpu::TextureFormat::Bgra8Unorm));
    }
}