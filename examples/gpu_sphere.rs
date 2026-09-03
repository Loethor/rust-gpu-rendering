// examples/gpu_sphere.rs
//
// Run with: cargo run --example gpu_sphere

use image::ColorType;

const SHADER: &str = r#"
struct Config {
    width: u32,
    height: u32,
}

@group(0) @binding(0)
var<storage, read_write> pixels: array<u32>;

@group(0) @binding(1)
var<uniform> config: Config;

fn pack_rgba8(r: f32, g: f32, b: f32) -> u32 {
    let ri = u32(clamp(r, 0.0, 1.0) * 255.0 + 0.5);
    let gi = u32(clamp(g, 0.0, 1.0) * 255.0 + 0.5);
    let bi = u32(clamp(b, 0.0, 1.0) * 255.0 + 0.5);
    return ri | (gi << 8u) | (bi << 16u) | (255u << 24u);
}

fn sky_color(rd: vec3<f32>) -> vec3<f32> {
    let t = 0.5 * (rd.y + 1.0);
    return mix(vec3<f32>(1.0, 1.0, 1.0), vec3<f32>(0.5, 0.7, 1.0), t);
}

// The EXACT same math as src/scene.rs, translated to WGSL.
// Same names: oc, half_b, discriminant.
fn hit_sphere(ro: vec3<f32>, rd: vec3<f32>, center: vec3<f32>, radius: f32) -> f32 {
    let oc = ro - center;
    let a = dot(rd, rd);
    let half_b = dot(oc, rd);
    let c = dot(oc, oc) - radius * radius;

    let discriminant = half_b * half_b - a * c;
    if (discriminant < 0.0) {
        return -1.0;
    }

    let sqrt_d = sqrt(discriminant);
    var t = (-half_b - sqrt_d) / a;
    if (t < 0.0) {
        t = (-half_b + sqrt_d) / a;
    }
    if (t < 0.0) {
        return -1.0;
    }
    return t;
}

@compute @workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    if (id.x >= config.width || id.y >= config.height) {
        return;
    }

    // ---- camera (same as Camera::ray_through) ----
    let aspect = f32(config.width) / f32(config.height);
    let u = (f32(id.x) / f32(config.width)) * 2.0 - 1.0;
    let v = -((f32(id.y) / f32(config.height)) * 2.0 - 1.0);

    let ro = vec3<f32>(0.0, 0.0, 0.0);
    let rd = normalize(vec3<f32>(u * aspect, v, -1.0));

    // ---- scene (hardcoded for now) ----
    let center = vec3<f32>(0.0, 0.0, -1.0);
    let radius = 0.5;
    let albedo = vec3<f32>(0.9, 0.45, 0.2);
    let light_dir = normalize(vec3<f32>(-0.5, 0.7, 0.6));

    // ---- shade ----
    let t = hit_sphere(ro, rd, center, radius);
    var color: vec3<f32>;

    if (t < 0.0) {
        color = sky_color(rd);
    } else {
        let hit_point = ro + rd * t;
        let normal = normalize(hit_point - center);

        let diffuse = max(dot(normal, light_dir), 0.0);
        let ambient = 0.15;
        let intensity = ambient + (1.0 - ambient) * diffuse;
        color = albedo * intensity;
    }

    // gamma, same as colors::to_rgba8 on the CPU
    color = pow(color, vec3<f32>(1.0 / 2.2));

    let index = id.y * config.width + id.x;
    pixels[index] = pack_rgba8(color.r, color.g, color.b);
}
"#;

fn main() {
    pollster::block_on(run());
}

async fn run() {
    let width = 800u32;
    let height = 600u32;
    let buffer_size = (width * height) as u64 * 4;

    // ---- Step 1: pick a GPU and open a channel to it ----
    // wgpu 30: Instance::default() is the cleanest way to get an instance with all backends enabled.
    let instance = wgpu::Instance::default();

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: None,
            force_fallback_adapter: false,
            apply_limit_buckets: false, // wgpu 30 added this field
        })
        .await
        .expect("No GPU adapter found");
    println!("Using adapter: {:?}", adapter.get_info());

    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor {
            label: Some("device"),
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::downlevel_defaults(),
            ..Default::default()
        })
        .await
        .expect("Failed to create GPU device");

    // ---- Step 2: allocate GPU memory ----
    let pixel_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("pixel buffer"),
        size: buffer_size,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });
    let readback_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("readback buffer"),
        size: buffer_size,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let config_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("config buffer"),
        size: 8,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let mut config_bytes = [0u8; 8];
    config_bytes[0..4].copy_from_slice(&width.to_ne_bytes());
    config_bytes[4..8].copy_from_slice(&height.to_ne_bytes());
    queue.write_buffer(&config_buffer, 0, &config_bytes);

    // ---- Step 3: compile the shader ----
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("gradient shader"),
        source: wgpu::ShaderSource::Wgsl(SHADER.into()),
    });

    // ---- Step 4: layout -> bind group -> pipeline ----
    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("bind group layout"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("pipeline layout"),
        bind_group_layouts: &[Some(&bind_group_layout)], // wgpu 30 expects Option
        immediate_size: 0, // wgpu 30 replaced push_constant_ranges with immediate_size
    });

    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("gradient pipeline"),
        layout: Some(&pipeline_layout),
        module: &shader,
        entry_point: Some("main"),
        compilation_options: Default::default(),
        cache: None,
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("bind group"),
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: pixel_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: config_buffer.as_entire_binding(),
            },
        ],
    });

    // ---- Step 5: record commands, run them ----
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("encoder"),
    });
    {
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("compute pass"),
            timestamp_writes: None,
        });
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &bind_group, &[]);
        pass.dispatch_workgroups((width + 15) / 16, (height + 15) / 16, 1);
    }
    encoder.copy_buffer_to_buffer(&pixel_buffer, 0, &readback_buffer, 0, buffer_size);
    queue.submit(Some(encoder.finish()));

    // ---- Step 6: read back and save ----
    let slice = readback_buffer.slice(..);
    let (tx, rx) = std::sync::mpsc::channel();
    slice.map_async(wgpu::MapMode::Read, move |r| tx.send(r).unwrap());

    // wgpu 30: Maintain was renamed to PollType
    device
        .poll(wgpu::PollType::Wait {
            submission_index: None,
            timeout: None,
        })
        .unwrap();

    rx.recv().unwrap().expect("Failed to map buffer");

    // wgpu 30: get_mapped_range now returns a Result
    let data = slice.get_mapped_range().unwrap();

    image::save_buffer("gpu_sphere.png", &data, width, height, ColorType::Rgba8)
        .expect("Failed to save gpu_sphere.png");
    println!("Saved gpu_sphere.png");
}
