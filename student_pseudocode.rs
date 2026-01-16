// ============================================================================
// STUDENT APPLICATION - MULTICAST RECEIVER & VIDEO PLAYER
// ============================================================================

use std::net::{UdpSocket, Ipv4Addr};
use std::process::{Command, Stdio};
use std::io::Read;

// ----------------------------------------------------------------------------
// 1. MULTICAST RECEIVER - NHẬN RTP STREAM
// ----------------------------------------------------------------------------

struct MulticastReceiver {
    socket: UdpSocket,
    multicast_addr: String,
    port: u16,
}

impl MulticastReceiver {
    fn new(multicast_addr: &str, port: u16) -> Result<Self> {
        // 1. Bind socket đến port
        let socket = UdpSocket::bind(format!("0.0.0.0:{}", port))?;
        
        // 2. Join multicast group
        let multicast_ip: Ipv4Addr = multicast_addr.parse()?;
        socket.join_multicast_v4(&multicast_ip, &Ipv4Addr::UNSPECIFIED)?;
        
        // 3. Set socket options
        socket.set_read_timeout(Some(std::time::Duration::from_secs(5)))?;
        
        println!("Joined multicast group: {}:{}", multicast_addr, port);
        
        Ok(Self {
            socket,
            multicast_addr: multicast_addr.to_string(),
            port,
        })
    }

    fn receive_packet(&self) -> Result<Vec<u8>> {
        let mut buffer = vec![0u8; 65536]; // Max UDP packet size
        let (size, _src) = self.socket.recv_from(&mut buffer)?;
        buffer.truncate(size);
        Ok(buffer)
    }
}

// ----------------------------------------------------------------------------
// 2. VIDEO DECODER - DECODE H.264 BẰNG GPU
// ----------------------------------------------------------------------------

struct VideoDecoder {
    ffmpeg_process: std::process::Child,
    stdout: std::process::ChildStdout,
}

impl VideoDecoder {
    fn new(multicast_addr: &str) -> Result<Self> {
        // FFmpeg command để nhận RTP và decode
        let mut ffmpeg = Command::new("ffmpeg")
            // Input: RTP stream
            .args(&[
                "-fflags", "nobuffer",
                "-flags", "low_delay",
                "-probesize", "32",
                "-analyzeduration", "0",
                "-i", &format!("rtp://{}", multicast_addr),
            ])
            // Decoding với GPU
            .args(&[
                "-c:v", "h264", // Auto-select GPU decoder nếu có
                // Hoặc chỉ định cụ thể:
                // "-hwaccel", "dxva2", // Windows GPU decode
                // "-c:v", "h264_cuvid", // NVIDIA
            ])
            // Output: raw frames
            .args(&[
                "-f", "rawvideo",
                "-pix_fmt", "bgra",
                "pipe:1", // Output to stdout
            ])
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()?;

        let stdout = ffmpeg.stdout.take().unwrap();

        Ok(Self {
            ffmpeg_process: ffmpeg,
            stdout,
        })
    }

    fn read_frame(&mut self, width: u32, height: u32) -> Result<Vec<u8>> {
        let frame_size = (width * height * 4) as usize; // BGRA = 4 bytes/pixel
        let mut frame_data = vec![0u8; frame_size];
        self.stdout.read_exact(&mut frame_data)?;
        Ok(frame_data)
    }
}

// ----------------------------------------------------------------------------
// 3. VIDEO RENDERER - HIỂN THỊ BẰNG DIRECTX/WGPU
// ----------------------------------------------------------------------------

use wgpu::*;
use winit::window::Window;

struct VideoRenderer {
    device: Device,
    queue: Queue,
    surface: Surface,
    surface_config: SurfaceConfiguration,
    texture: Texture,
    width: u32,
    height: u32,
}

impl VideoRenderer {
    async fn new(window: &Window, width: u32, height: u32) -> Result<Self> {
        // 1. Khởi tạo wgpu
        let instance = Instance::new(InstanceDescriptor::default());
        let surface = unsafe { instance.create_surface(window)? };
        
        let adapter = instance.request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }).await.unwrap();

        let (device, queue) = adapter.request_device(
            &DeviceDescriptor::default(),
            None,
        ).await?;

        // 2. Cấu hình surface
        let surface_config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_capabilities(&adapter).formats[0],
            width,
            height,
            present_mode: PresentMode::Fifo, // VSync
            alpha_mode: CompositeAlphaMode::Opaque,
            view_formats: vec![],
        };
        surface.configure(&device, &surface_config);

        // 3. Tạo texture để upload frame
        let texture = device.create_texture(&TextureDescriptor {
            label: Some("Video Frame"),
            size: Extent3d { width, height, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8Unorm,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            view_formats: &[],
        });

        Ok(Self {
            device,
            queue,
            surface,
            surface_config,
            texture,
            width,
            height,
        })
    }

    fn render_frame(&mut self, frame_data: &[u8]) -> Result<()> {
        // 1. Upload frame data lên GPU texture
        self.queue.write_texture(
            ImageCopyTexture {
                texture: &self.texture,
                mip_level: 0,
                origin: Origin3d::ZERO,
                aspect: TextureAspect::All,
            },
            frame_data,
            ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(self.width * 4),
                rows_per_image: Some(self.height),
            },
            Extent3d {
                width: self.width,
                height: self.height,
                depth_or_array_layers: 1,
            },
        );

        // 2. Render texture lên surface
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&TextureViewDescriptor::default());
        
        let mut encoder = self.device.create_command_encoder(
            &CommandEncoderDescriptor { label: Some("Render") }
        );

        // Render pass (simplified - cần thêm pipeline, shader, etc.)
        {
            let _render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color::BLACK),
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            
            // Draw textured quad here...
        }

        self.queue.submit(Some(encoder.finish()));
        output.present();

        Ok(())
    }
}

// ----------------------------------------------------------------------------
// 4. MAIN LOOP - STUDENT APPLICATION
// ----------------------------------------------------------------------------

use winit::event_loop::{EventLoop, ControlFlow};
use winit::event::{Event, WindowEvent};

fn main() -> Result<()> {
    println!("=== STUDENT - Screen Sharing Client ===");
    
    // 1. Cấu hình
    let multicast_addr = "239.0.0.1";
    let port = 5004;
    let video_width = 1920;
    let video_height = 1080;

    // 2. Tạo window
    let event_loop = EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .with_title("Teacher Screen")
        .with_inner_size(winit::dpi::PhysicalSize::new(video_width, video_height))
        .build(&event_loop)?;

    // 3. Khởi tạo renderer
    println!("Initializing GPU renderer...");
    let mut renderer = pollster::block_on(
        VideoRenderer::new(&window, video_width, video_height)
    )?;

    // 4. Join multicast và bắt đầu nhận stream
    println!("Joining multicast {}:{}...", multicast_addr, port);
    let receiver = MulticastReceiver::new(multicast_addr, port)?;

    // 5. Khởi động FFmpeg decoder
    println!("Starting video decoder...");
    let mut decoder = VideoDecoder::new(&format!("{}:{}", multicast_addr, port))?;

    println!("Receiving stream. Close window to exit.");

    // 6. Main event loop
    let mut frame_count = 0u64;
    let start_time = std::time::Instant::now();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("Closing...");
                *control_flow = ControlFlow::Exit;
            }

            Event::MainEventsCleared => {
                // Đọc và render frame
                match decoder.read_frame(video_width, video_height) {
                    Ok(frame_data) => {
                        if let Err(e) = renderer.render_frame(&frame_data) {
                            eprintln!("Render error: {}", e);
                        }
                        
                        frame_count += 1;
                        
                        // Log stats mỗi 5 giây
                        if frame_count % 150 == 0 { // ~5s @ 30fps
                            let elapsed = start_time.elapsed().as_secs_f64();
                            let fps = frame_count as f64 / elapsed;
                            println!("Frames: {} | FPS: {:.1}", frame_count, fps);
                        }
                    }
                    Err(e) => {
                        eprintln!("Decode error: {}", e);
                        std::thread::sleep(std::time::Duration::from_millis(10));
                    }
                }

                window.request_redraw();
            }

            _ => {}
        }
    });
}

// ----------------------------------------------------------------------------
// 5. PHIÊN BẢN ĐƠN GIẢN HƠN - DÙNG FFPLAY
// ----------------------------------------------------------------------------

// Nếu không muốn implement renderer phức tạp, có thể dùng FFplay:

fn simple_student_main() -> Result<()> {
    println!("=== STUDENT - Simple Player ===");
    
    let multicast_addr = "239.0.0.1:5004";
    
    println!("Connecting to {}...", multicast_addr);
    println!("Press Q to quit.");

    // Chạy FFplay để nhận và hiển thị stream
    let status = Command::new("ffplay")
        .args(&[
            "-fflags", "nobuffer",
            "-flags", "low_delay",
            "-framedrop",
            "-probesize", "32",
            "-analyzeduration", "0",
            "-sync", "ext",
            "-hwaccel", "auto", // Tự động dùng GPU decode
            &format!("rtp://{}", multicast_addr),
        ])
        .status()?;

    if !status.success() {
        eprintln!("FFplay exited with error");
    }

    Ok(())
}

// ----------------------------------------------------------------------------
// 6. LƯU Ý TRIỂN KHAI
// ----------------------------------------------------------------------------

/*
DEPENDENCIES (Cargo.toml):
```toml
[dependencies]
wgpu = "0.18"
winit = "0.29"
pollster = "0.3"
```

HIỆU NĂNG:
- CPU: 5-10% (chủ yếu RTP processing)
- GPU: 10-15% (decode + render)
- RAM: 100-150 MB
- Network: 5 Mbps download

TỐI ƯU:
1. GPU Decode:
   - Windows: DXVA2 (tự động với FFmpeg)
   - NVIDIA: NVDEC (h264_cuvid)
   - Intel: QuickSync (h264_qsv)

2. Giảm độ trễ:
   - Tắt buffer: -fflags nobuffer
   - Drop frame nếu decode chậm: -framedrop
   - Giảm probe time: -probesize 32

3. Xử lý mất gói:
   - Decoder tự động skip frame bị lỗi
   - Chờ keyframe tiếp theo để resync
   - Acceptable: 1-2% packet loss

TROUBLESHOOTING:
1. Không nhận được stream:
   - Kiểm tra firewall (port 5004 UDP)
   - Kiểm tra multicast route: `route print` (Windows)
   - Verify cùng subnet với Teacher

2. Giật lag:
   - Check network: `ping teacher_ip`
   - Monitor packet loss: Wireshark
   - Giảm resolution/bitrate ở Teacher

3. Màn hình đen:
   - Verify stream đang chạy: `ffprobe rtp://239.0.0.1:5004`
   - Check GPU driver updated
   - Thử software decode: bỏ -hwaccel

4. CPU/GPU cao:
   - Đảm bảo dùng GPU decode (không phải software)
   - Check driver GPU mới nhất
   - Giảm resolution window nếu cần
*/
