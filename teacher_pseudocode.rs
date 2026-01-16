// ============================================================================
// TEACHER APPLICATION - SCREEN CAPTURE & MULTICAST STREAMING
// ============================================================================

use std::process::{Command, Stdio};
use std::io::Write;
use std::net::UdpSocket;
use windows::Win32::Graphics::Dxgi::*;
use windows::Win32::Graphics::Direct3D11::*;

// ----------------------------------------------------------------------------
// 1. DXGI DESKTOP DUPLICATION - CAPTURE MÀN HÌNH
// ----------------------------------------------------------------------------

struct ScreenCapturer {
    device: ID3D11Device,
    context: ID3D11DeviceContext,
    duplication: IDXGIOutputDuplication,
    staging_texture: ID3D11Texture2D,
    width: u32,
    height: u32,
}

impl ScreenCapturer {
    fn new() -> Result<Self> {
        // 1. Tạo D3D11 Device
        let mut device = None;
        let mut context = None;
        unsafe {
            D3D11CreateDevice(
                None, // Adapter mặc định
                D3D_DRIVER_TYPE_HARDWARE,
                None,
                D3D11_CREATE_DEVICE_FLAG(0),
                None,
                D3D11_SDK_VERSION,
                Some(&mut device),
                None,
                Some(&mut context),
            )?;
        }
        
        let device = device.unwrap();
        let context = context.unwrap();

        // 2. Lấy DXGI Output (Monitor)
        let dxgi_device: IDXGIDevice = device.cast()?;
        let adapter = unsafe { dxgi_device.GetAdapter()? };
        let output = unsafe { adapter.EnumOutputs(0)? }; // Monitor đầu tiên
        let output1: IDXGIOutput1 = output.cast()?;

        // 3. Tạo Desktop Duplication
        let duplication = unsafe {
            output1.DuplicateOutput(&device)?
        };

        // 4. Lấy thông tin màn hình
        let desc = unsafe { output.GetDesc()? };
        let width = (desc.DesktopCoordinates.right - desc.DesktopCoordinates.left) as u32;
        let height = (desc.DesktopCoordinates.bottom - desc.DesktopCoordinates.top) as u32;

        // 5. Tạo staging texture để đọc dữ liệu từ GPU về CPU
        let staging_desc = D3D11_TEXTURE2D_DESC {
            Width: width,
            Height: height,
            MipLevels: 1,
            ArraySize: 1,
            Format: DXGI_FORMAT_B8G8R8A8_UNORM, // BGRA
            SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
            Usage: D3D11_USAGE_STAGING,
            BindFlags: D3D11_BIND_FLAG(0),
            CPUAccessFlags: D3D11_CPU_ACCESS_READ,
            MiscFlags: D3D11_RESOURCE_MISC_FLAG(0),
        };

        let mut staging_texture = None;
        unsafe {
            device.CreateTexture2D(&staging_desc, None, Some(&mut staging_texture))?;
        }

        Ok(Self {
            device,
            context,
            duplication,
            staging_texture: staging_texture.unwrap(),
            width,
            height,
        })
    }

    fn capture_frame(&mut self) -> Result<Vec<u8>> {
        // 1. Acquire next frame từ DXGI
        let mut frame_info = DXGI_OUTDUPL_FRAME_INFO::default();
        let mut desktop_resource = None;
        
        unsafe {
            self.duplication.AcquireNextFrame(
                16, // Timeout 16ms (cho 60fps)
                &mut frame_info,
                &mut desktop_resource,
            )?;
        }

        // 2. Kiểm tra có frame mới không
        if frame_info.LastPresentTime == 0 {
            unsafe { self.duplication.ReleaseFrame()?; }
            return Err("No new frame");
        }

        // 3. Cast resource thành texture
        let desktop_texture: ID3D11Texture2D = 
            desktop_resource.unwrap().cast()?;

        // 4. Copy từ GPU texture sang staging texture
        unsafe {
            self.context.CopyResource(
                &self.staging_texture,
                &desktop_texture,
            );
        }

        // 5. Map staging texture để đọc dữ liệu
        let mut mapped = D3D11_MAPPED_SUBRESOURCE::default();
        unsafe {
            self.context.Map(
                &self.staging_texture,
                0,
                D3D11_MAP_READ,
                0,
                Some(&mut mapped),
            )?;
        }

        // 6. Copy dữ liệu BGRA từ GPU về RAM
        let row_pitch = mapped.RowPitch as usize;
        let data_size = (self.height as usize) * row_pitch;
        let mut frame_data = vec![0u8; data_size];

        unsafe {
            std::ptr::copy_nonoverlapping(
                mapped.pData as *const u8,
                frame_data.as_mut_ptr(),
                data_size,
            );
        }

        // 7. Unmap và release frame
        unsafe {
            self.context.Unmap(&self.staging_texture, 0);
            self.duplication.ReleaseFrame()?;
        }

        Ok(frame_data)
    }
}

// ----------------------------------------------------------------------------
// 2. FFMPEG ENCODER - ENCODE H.264 VÀ STREAM RTP
// ----------------------------------------------------------------------------

struct VideoEncoder {
    ffmpeg_process: std::process::Child,
    stdin: std::process::ChildStdin,
    width: u32,
    height: u32,
    fps: u32,
}

impl VideoEncoder {
    fn new(width: u32, height: u32, fps: u32, multicast_addr: &str) -> Result<Self> {
        // FFmpeg command để encode và stream
        let mut ffmpeg = Command::new("ffmpeg")
            // Input: raw BGRA frames từ stdin
            .args(&[
                "-f", "rawvideo",
                "-pixel_format", "bgra",
                "-video_size", &format!("{}x{}", width, height),
                "-framerate", &fps.to_string(),
                "-i", "pipe:0", // Đọc từ stdin
            ])
            // Encoding settings
            .args(&[
                "-c:v", "libx264",
                "-preset", "ultrafast", // Hoặc "veryfast" cho chất lượng tốt hơn
                "-tune", "zerolatency",
                "-profile:v", "baseline",
                "-level", "3.1",
                "-pix_fmt", "yuv420p",
            ])
            // GOP và keyframe
            .args(&[
                "-g", "60", // Keyframe mỗi 2 giây (60 frames @ 30fps)
                "-keyint_min", "60",
            ])
            // Bitrate control
            .args(&[
                "-b:v", "4M",
                "-maxrate", "5M",
                "-bufsize", "2M",
            ])
            // Output: RTP multicast
            .args(&[
                "-f", "rtp_mpegts",
                &format!("rtp://{}?ttl=1", multicast_addr),
            ])
            // Logging
            .args(&[
                "-loglevel", "warning",
            ])
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::inherit())
            .spawn()?;

        let stdin = ffmpeg.stdin.take().unwrap();

        Ok(Self {
            ffmpeg_process: ffmpeg,
            stdin,
            width,
            height,
            fps,
        })
    }

    fn encode_frame(&mut self, frame_data: &[u8]) -> Result<()> {
        // Ghi raw BGRA frame vào stdin của FFmpeg
        self.stdin.write_all(frame_data)?;
        self.stdin.flush()?;
        Ok(())
    }

    fn stop(&mut self) -> Result<()> {
        drop(self.stdin.take()); // Close stdin
        self.ffmpeg_process.wait()?;
        Ok(())
    }
}

// ----------------------------------------------------------------------------
// 3. MAIN LOOP - TEACHER APPLICATION
// ----------------------------------------------------------------------------

fn main() -> Result<()> {
    println!("=== TEACHER - Screen Sharing Server ===");
    
    // 1. Khởi tạo screen capturer
    println!("Initializing DXGI screen capture...");
    let mut capturer = ScreenCapturer::new()?;
    println!("Screen: {}x{}", capturer.width, capturer.height);

    // 2. Khởi tạo video encoder
    println!("Starting FFmpeg encoder...");
    let multicast_addr = "239.0.0.1:5004";
    let fps = 30;
    let mut encoder = VideoEncoder::new(
        capturer.width,
        capturer.height,
        fps,
        multicast_addr,
    )?;
    println!("Streaming to: {}", multicast_addr);

    // 3. Main capture loop
    println!("Streaming started. Press Ctrl+C to stop.");
    let frame_duration = std::time::Duration::from_millis(1000 / fps as u64);
    let mut frame_count = 0u64;
    let start_time = std::time::Instant::now();

    loop {
        let loop_start = std::time::Instant::now();

        // Capture frame từ màn hình
        match capturer.capture_frame() {
            Ok(frame_data) => {
                // Encode và stream frame
                if let Err(e) = encoder.encode_frame(&frame_data) {
                    eprintln!("Encode error: {}", e);
                    break;
                }
                
                frame_count += 1;
                
                // Log stats mỗi 5 giây
                if frame_count % (fps as u64 * 5) == 0 {
                    let elapsed = start_time.elapsed().as_secs_f64();
                    let actual_fps = frame_count as f64 / elapsed;
                    println!("Frames: {} | FPS: {:.1} | Bitrate: ~4-5 Mbps", 
                             frame_count, actual_fps);
                }
            }
            Err(e) => {
                // Không có frame mới, bỏ qua
                if e.to_string().contains("No new frame") {
                    std::thread::sleep(std::time::Duration::from_millis(1));
                    continue;
                } else {
                    eprintln!("Capture error: {}", e);
                    break;
                }
            }
        }

        // Frame pacing - đảm bảo đúng FPS
        let elapsed = loop_start.elapsed();
        if elapsed < frame_duration {
            std::thread::sleep(frame_duration - elapsed);
        }
    }

    // 4. Cleanup
    println!("Stopping encoder...");
    encoder.stop()?;
    println!("Streaming stopped.");

    Ok(())
}

// ----------------------------------------------------------------------------
// 4. LƯU Ý QUAN TRỌNG
// ----------------------------------------------------------------------------

/*
HIỆU NĂNG:
- CPU usage: ~15-25% (1 core Intel i5+)
- RAM: ~200-300 MB
- Network: 5 Mbps (cố định, không tăng theo số student)
- Capture overhead: Minimal (~5% GPU)

TỐI ƯU:
1. Nếu CPU cao: Dùng GPU encoder (NVENC/QuickSync)
   - Thay "-c:v libx264" bằng "-c:v h264_nvenc" (NVIDIA)
   - Hoặc "-c:v h264_qsv" (Intel QuickSync)

2. Nếu muốn chất lượng tốt hơn:
   - Đổi preset từ "ultrafast" → "veryfast"
   - Tăng bitrate lên 6-8 Mbps
   - Trade-off: CPU tăng ~10-15%

3. Nếu màn hình lớn (4K):
   - Downscale về 1920x1080: thêm "-vf scale=1920:1080"
   - Hoặc giảm FPS xuống 25

TROUBLESHOOTING:
- Nếu FFmpeg không tìm thấy: Cài FFmpeg và thêm vào PATH
- Nếu multicast không hoạt động: Kiểm tra firewall và switch
- Nếu màu sai: Đảm bảo pixel format đúng (bgra → yuv420p)
*/
