# HƯỚNG DẪN SỬ DỤNG ỨNG DỤNG

## 🚀 CÀI ĐẶT

### 1. Cài đặt FFmpeg (BẮT BUỘC)

**Windows:**
```powershell
# Option 1: Auto-download script (khuyến nghị)
.\download_ffmpeg.ps1

# Option 2: Chocolatey
choco install ffmpeg

# Option 3: Manual
# Download từ: https://ffmpeg.org/download.html
# Copy ffmpeg.exe và ffplay.exe vào src-tauri/resources/
```

**macOS:**
```bash
# Option 1: Homebrew (khuyến nghị)
brew install ffmpeg

# Option 2: Auto-install script
chmod +x download_ffmpeg.sh
./download_ffmpeg.sh
```

**Linux:**
```bash
# Ubuntu/Debian
sudo apt install ffmpeg

# Fedora
sudo dnf install ffmpeg

# Arch
sudo pacman -S ffmpeg
```

### 2. Cài đặt Dependencies

```bash
# Install Node.js dependencies
npm install

# Install Rust dependencies (tự động khi build)
```

### 3. Cấu hình Firewall

**Teacher Machine:**
```powershell
netsh advfirewall firewall add rule name="Screen Share Teacher" ^
  dir=out action=allow protocol=UDP remoteip=239.0.0.1 remoteport=5004
```

**Student Machines:**
```powershell
netsh advfirewall firewall add rule name="Screen Share Student" ^
  dir=in action=allow protocol=UDP localip=239.0.0.1 localport=5004
```

## 🏃 CHẠY ỨNG DỤNG

### Development Mode

```bash
npm run tauri dev
```

### Build Production

```bash
npm run tauri build
```

Binary sẽ được tạo trong `src-tauri/target/release/`

## 📖 HƯỚNG DẪN SỬ DỤNG

### Teacher Mode

1. **Khởi động app** và chọn "Teacher Mode"
2. **Cấu hình**:
   - Multicast Address: `239.0.0.1:5004` (mặc định)
   - Bitrate: Chọn từ 2M đến 5M (khuyến nghị 4M)
3. **Click "Start Streaming"**
4. Màn hình sẽ được capture và stream tới multicast address
5. **Click "Stop Streaming"** để dừng

### Student Mode

1. **Khởi động app** và chọn "Student Mode"
2. **Nhập Multicast Address**: `239.0.0.1:5004` (phải giống Teacher)
3. **Click "Connect to Teacher"**
4. Cửa sổ FFplay sẽ mở và hiển thị màn hình Teacher
5. **Press 'Q'** trong cửa sổ FFplay để thoát

## 🏗️ KIẾN TRÚC CODE

### Backend (Rust)

```
src-tauri/src/
├── lib.rs              # Main entry, Tauri commands
├── screen_capture.rs   # DXGI Desktop Duplication
└── streaming.rs        # FFmpeg integration
```

**Modules:**

1. **screen_capture.rs**
   - `ScreenCapturer::new()` - Khởi tạo DXGI
   - `capture_frame()` - Capture 1 frame BGRA

2. **streaming.rs**
   - `StreamingServer::new()` - Khởi tạo server
   - `start()` - Spawn FFmpeg process
   - `send_frame()` - Gửi frame vào FFmpeg stdin
   - `stop()` - Dừng streaming

3. **lib.rs - Tauri Commands**
   - `start_teacher_stream()` - Bắt đầu capture + stream
   - `stop_teacher_stream()` - Dừng streaming
   - `start_student_view()` - Mở FFplay
   - `check_ffmpeg()` - Kiểm tra FFmpeg

### Frontend (React + TypeScript)

```
src/
├── App.tsx    # Main UI component
├── App.css    # Styles
└── main.tsx   # Entry point
```

**UI Modes:**
- `select` - Chọn Teacher/Student
- `teacher` - Teacher control panel
- `student` - Student viewer

## 🔧 TROUBLESHOOTING

### FFmpeg not found

**Triệu chứng:** App báo "FFmpeg not found"

**Giải pháp:**
1. Cài FFmpeg: `choco install ffmpeg`
2. Verify: `ffmpeg -version`
3. Restart app

### Cannot start streaming (Windows)

**Triệu chứng:** Error khi click "Start Streaming"

**Giải pháp:**
1. Chạy app với quyền Administrator
2. Check firewall rules
3. Verify GPU driver updated

### Student không nhận được stream

**Triệu chứng:** FFplay mở nhưng màn hình đen

**Giải pháp:**
1. Verify cùng subnet: `ipconfig`
2. Test ping Teacher: `ping <teacher_ip>`
3. Check firewall: `netsh advfirewall show allprofiles`
4. Verify multicast route: `route print`

### Giật lag

**Triệu chứng:** Video bị giật, không mượt

**Giải pháp:**
1. Giảm bitrate xuống 3M hoặc 2M
2. Check network: `ping -t <teacher_ip>`
3. Verify Gigabit connection
4. Close other network-heavy apps

## 🎯 TỐI ƯU HIỆU NĂNG

### GPU Encoding (Teacher)

Để giảm CPU usage, sửa `streaming.rs`:

```rust
// Thay libx264 bằng:
// NVIDIA:
.args(&["-c:v", "h264_nvenc", "-preset", "p1", "-tune", "ll"])

// Intel QuickSync:
.args(&["-c:v", "h264_qsv", "-preset", "veryfast"])
```

### Giảm Resolution

Thêm vào `streaming.rs` trước encoding:

```rust
.args(&["-vf", "scale=1280:720"])
```

### Giảm FPS

Sửa trong `lib.rs`:

```rust
let fps = 25u32; // Thay vì 30
```

## 📊 MONITORING

### Teacher Stats

Xem console output:
```
Frames: 150 | FPS: 30.1
Frames: 300 | FPS: 30.0
```

### Network Usage

```powershell
# Monitor network
netstat -e 1

# Expected: ~5 Mbps upload (Teacher)
```

### CPU/GPU Usage

- Task Manager → Performance
- Teacher: CPU ~15-25% (libx264) hoặc ~5% (NVENC)
- Student: CPU ~5-10%, GPU ~10-15%

## 🔐 SECURITY

### LAN Only

- TTL = 1: Packets không route ra ngoài LAN
- Multicast IP 239.0.0.1: Class D, LAN-safe
- Không cần authentication (trust LAN)

### Production Deployment

Nếu cần security:
1. Thêm encryption (AES)
2. Thêm authentication token
3. Whitelist IP addresses
4. VLAN isolation

## 📝 NOTES

### Windows Only

- DXGI Desktop Duplication chỉ có trên Windows
- Để hỗ trợ macOS/Linux, cần implement:
  - macOS: AVFoundation screen capture
  - Linux: X11/Wayland capture

### FFmpeg Dependency

- App phụ thuộc FFmpeg external binary
- Để standalone: Bundle FFmpeg vào app
- Hoặc: Implement H.264 encoder native (libx264 binding)

### Multicast Support

- Cần switch hỗ trợ IGMP
- Hầu hết switch hiện đại đều hỗ trợ
- Nếu không: Dùng unicast (1-to-1) thay vì multicast

## 🚀 NEXT STEPS

### Tính năng có thể thêm:

1. **Audio streaming** - Capture và stream audio
2. **Recording** - Ghi lại session
3. **Control channel** - Teacher điều khiển Students
4. **Chat** - Text chat giữa Teacher và Students
5. **Screen annotation** - Vẽ trên màn hình
6. **Multiple monitors** - Chọn monitor để share
7. **Quality presets** - Low/Medium/High/Ultra
8. **Network stats** - Hiển thị bandwidth, latency, packet loss
9. **Student list** - Xem danh sách Students đang xem
10. **Pause/Resume** - Tạm dừng streaming

### Code improvements:

1. Error handling tốt hơn
2. Logging system
3. Configuration file
4. Auto-reconnect
5. Graceful shutdown
6. Unit tests
7. Integration tests
