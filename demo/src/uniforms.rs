use wgpu::util::{DeviceExt, BufferInitDescriptor};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, Default)]
pub struct UniformsRaw {
    pub i_time: f32,
}

pub struct Uniforms {
    pub layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
    #[allow(dead_code)]
    buffer: wgpu::Buffer,
    pub raw: UniformsRaw,
}

impl Uniforms {
    pub fn new(device: &wgpu::Device) -> Self {
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Uniform Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::UniformBuffer {
                        dynamic: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ]
        });
        let raw = UniformsRaw::default();
        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&[raw]),
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Uniform Bind Group"),
            layout: &layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                }
            ]
        });

        Self {
            layout,
            bind_group,
            buffer,
            raw,
        }
    }

    pub fn update(&mut self, queue: &wgpu::Queue, dt: f32) {
        self.raw.i_time += dt;
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[self.raw]));
    }
}

