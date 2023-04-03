// Includes - to cleanup
extern crate kiss3d;
extern crate rand;

use kiss3d::camera::Camera;
use kiss3d::context::Context;
use kiss3d::renderer::Renderer;
use kiss3d::resource::{
    AllocationType, BufferType, Effect, GPUVec, ShaderAttribute, ShaderUniform,
};
use kiss3d::nalgebra::{Matrix4, Point3};

// Temporary wgpu test.
use wgpu::util::DeviceExt;
use std::{borrow::Cow};


/// Structure which manages the display of long-living points.
pub struct PointsCloudRenderer {

    shader: Effect,
    pos: ShaderAttribute<Point3<f32>>,
    //img_output: ShaderAttribute<Matrix4<f32>>,
    color: ShaderAttribute<Point3<f32>>,
    proj: ShaderUniform<Matrix4<f32>>,
    view: ShaderUniform<Matrix4<f32>>,
    colored_points: GPUVec<Point3<f32>>,
    modified_points: GPUVec<Point3<f32>>,
    point_size: f32,
    noise: Effect,
}

impl PointsCloudRenderer {
    
    /// Creation of a new manager.
    pub fn new(point_size: f32) -> PointsCloudRenderer {
        let mut shader = Effect::new_from_str(VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC);

        shader.use_program();


        let noise = Effect::new_from_str(NOISE_SRC, "");


        PointsCloudRenderer {
            colored_points: GPUVec::new(Vec::new(), BufferType::Array, AllocationType::StreamDraw),
            modified_points: GPUVec::new(Vec::new(), BufferType::ElementArray, AllocationType::StreamDraw),
            pos: shader.get_attrib::<Point3<f32>>("position").unwrap(),
            color: shader.get_attrib::<Point3<f32>>("color").unwrap(),
            proj: shader.get_uniform::<Matrix4<f32>>("proj").unwrap(),
            view: shader.get_uniform::<Matrix4<f32>>("view").unwrap(),
            shader,
            point_size,
            noise,
        }
    }

    pub fn push(&mut self, point: Point3<f32>, color: Point3<f32>) {
        if let Some(colored_points) = self.colored_points.data_mut() {
            colored_points.push(point);
            colored_points.push(color);
        }
    }

    pub fn num_points(&self) -> usize {
        self.colored_points.len() / 2
    }
}

impl Renderer for PointsCloudRenderer {
    /// Actually draws the points.
    fn render(&mut self, pass: usize, camera: &mut dyn Camera) {
        if self.colored_points.len() == 0 {
            return;
        }

        self.shader.use_program();
        self.color.enable();
        self.pos.enable();
        self.pos.bind_sub_buffer(&mut self.colored_points, 1, 0);


        if let Some(colored_points) = self.colored_points.data_mut() {

            // This is SUPER inefficient but at least it should work!
            // copying points to vector
            let support_vec = colored_points.iter()
                .map(|point| point.x)
                .collect();
            let out_vec = pollster::block_on(execute_gpu_noise(support_vec)).unwrap();
            //println!("debug out {:?}", out_vec[0]);
            colored_points.iter_mut().zip(out_vec.iter())
                .for_each(|(point, &noise)| point.x = noise);

            //println!("debug: {:?}", colored_points[0]);
        }



        camera.upload(pass, &mut self.proj, &mut self.view);

        self.color.bind_sub_buffer(&mut self.colored_points, 1, 1);

        let ctxt = Context::get();
        ctxt.point_size(self.point_size);
        ctxt.draw_arrays(Context::POINTS, 0, (self.colored_points.len() / 2) as i32);

        self.pos.disable();
        self.color.disable();
    }
}






async fn execute_gpu_noise(in_vec : Vec<f32>) -> Option<Vec<f32>> {

    // Instantiates instance of WebGPU
    let instance = wgpu::Instance::default();

    // `request_adapter` instantiates the general connection to the GPU
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await?;

    // `request_device` instantiates the feature specific connection to the GPU, defining some parameters,
    //  `features` being the available features.
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::downlevel_defaults(),
            },
            None,
        )
        .await
        .unwrap();

    let info = adapter.get_info();
    // skip this on LavaPipe temporarily
    if info.vendor == 0x10005 {
        return None;
    }

    execute_gpu_inner(&device, &queue, in_vec).await
}



async fn execute_gpu_inner(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    in_vec : Vec<f32>,
) -> Option<Vec<f32>> {
    // Loads the shader from WGSL
    let cs_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
    });

    let numbers = &in_vec;

    // Gets the size in bytes of the buffer.
    let slice_size = 128; // numbers.len() * std::mem::size_of::<u32>();
    let size = slice_size as wgpu::BufferAddress;

    // Instantiates buffer without data.
    // `usage` of buffer specifies how it can be used:
    //   `BufferUsages::MAP_READ` allows it to be read (outside the shader).
    //   `BufferUsages::COPY_DST` allows it to be the destination of the copy.
    let staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    // Instantiates buffer with data (`numbers`).
    // Usage allowing the buffer to be:
    //   A storage buffer (can be bound within a bind group and thus available to a shader).
    //   The destination of a copy.
    //   The source of a copy.
    let storage_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Storage Buffer"),
        contents: bytemuck::cast_slice(numbers),
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_DST
            | wgpu::BufferUsages::COPY_SRC,
    });

    // A bind group defines how buffers are accessed by shaders.
    // It is to WebGPU what a descriptor set is to Vulkan.
    // `binding` here refers to the `binding` of a buffer in the shader (`layout(set = 0, binding = 0) buffer`).

    // A pipeline specifies the operation of a shader

    // Instantiates the pipeline.
    let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: None,
        layout: None,
        module: &cs_module,
        entry_point: "main",
    });

    // Instantiates the bind group, once again specifying the binding of buffers.
    let bind_group_layout = compute_pipeline.get_bind_group_layout(0);
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: storage_buffer.as_entire_binding(),
        }],
    });

    // A command encoder executes one or many pipelines.
    // It is to WebGPU what a command buffer is to Vulkan.
    let mut encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    {
        let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
        cpass.set_pipeline(&compute_pipeline);
        cpass.set_bind_group(0, &bind_group, &[]);
        cpass.insert_debug_marker("compute noise generation");
        cpass.dispatch_workgroups(/*numbers.len()*/ 128 as u32, 1, 1); // Number of cells to run, the (x,y,z) size of item being processed
    }
    // Sets adds copy operation to command encoder.
    // Will copy data from storage buffer on GPU to staging buffer on CPU.
    encoder.copy_buffer_to_buffer(&storage_buffer, 0, &staging_buffer, 0, size);

    // Submits command encoder for processing
    queue.submit(Some(encoder.finish()));

    // Note that we're not calling `.await` here.
    let buffer_slice = staging_buffer.slice(..);
    // Sets the buffer up for mapping, sending over the result of the mapping back to us when it is finished.
    let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();
    buffer_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());

    // Poll the device in a blocking manner so that our future resolves.
    // In an actual application, `device.poll(...)` should
    // be called in an event loop or on another thread.
    device.poll(wgpu::Maintain::Wait);

    // Awaits until `buffer_future` can be read from
    if let Some(Ok(())) = receiver.receive().await {
        // Gets contents of buffer
        let data = buffer_slice.get_mapped_range();
        // Since contents are got in bytes, this converts these bytes back to u32
        let result = bytemuck::cast_slice(&data).to_vec();

        // With the current interface, we have to make sure all mapped views are
        // dropped before we unmap the buffer.
        drop(data);
        staging_buffer.unmap(); // Unmaps buffer from memory
        // If you are familiar with C++ these 2 lines can be thought of similarly to:
        //   delete myPointer;
        //   myPointer = NULL;
        // It effectively frees the memory

        // Returns data from buffer
        Some(result)
    } else {
        panic!("failed to run compute on gpu!")
    }
}


// ??
const VERTEX_SHADER_SRC: &str = "#version 100
    attribute vec3 position;
    attribute vec3 color;
    varying   vec3 Color;
    varying in vec3 position_after;
    uniform   mat4 proj;
    uniform   mat4 view;
    void main() {
        gl_Position = proj * view * vec4(position + position_after, 1.0);
        Color = color;
    }";

const FRAGMENT_SHADER_SRC: &str = "#version 100
#ifdef GL_FRAGMENT_PRECISION_HIGH
   precision highp float;
#else
   precision mediump float;
#endif
    varying vec3 Color;
    void main() {
        gl_FragColor = vec4(Color, 1.0);
    }";

const NOISE_SRC: &str = "#version 430
    in vec3 position;
    out vec3 position_after;
    layout(rgba32f, binding = 2) writeonly uniform image2D img_output;

    float random3 (vec3 st) {
    return fract(sin(dot(st.xyz,
                         vec3(12.9898,78.233,49.391)))*
        43758.5453123);
    }

    void main() {
        //position_after = vec3(0., 0., 0.01 * random3(position));
        ivec2 coord = ivec2(gl_GlobalInvocationID.xy);
        vec4 color = vec4(0., 0., 0.01 * random3(position), 1.);
        imageStore(img_output, coord, color);
        //position_after = vec3(0.,0.,0.5);
    }";