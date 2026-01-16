# ⚠️ LƯU Ý QUAN TRỌNG - macOS

## 🚨 SCREEN CAPTURE TRÊN macOS

App hiện tại sử dụng **DXGI Desktop Duplication API** - chỉ có trên **Windows**.

### Trên macOS:

#### ✅ Student Mode - HOẠT ĐỘNG BÌNH THƯỜNG
- Nhận và xem stream từ Teacher
- Sử dụng FFplay (qua Homebrew)
- Không cần thay đổi gì

#### ❌ Teacher Mode - KHÔNG HOẠT ĐỘNG
- DXGI không có trên macOS
- Cần implement AVFoundation screen capture
- Hoặc dùng FFmpeg command line

---

## 🔧 WORKAROUND - TEACHER MODE TRÊN macOS

### Option 1: Dùng FFmpeg Command Line

```bash
# Capture màn hình và stream
ffmpeg -f avfoundation -i "1:0" \
  -c:v libx264 -preset ultrafast -tune zerolatency \
  -profile:v baseline -pix_fmt yuv420p \
  -g 60 -b:v 4M -maxrate 5M -bufsize 2M \
  -f rtp_mpegts rtp://239.0.0.1:5004?ttl=1
```

**Giải thích:**
- `-f avfoundation`: Input format cho macOS
- `-i "1:0"`: Screen 1, Audio device 0
- Các tham số khác giống Windows

### Option 2: List Available Screens

```bash
# Xem danh sách screens và audio devices
ffmpeg -f avfoundation -list_devices true -i ""

# Output example:
# [AVFoundation indev @ 0x...] AVFoundation video devices:
# [AVFoundation indev @ 0x...] [0] FaceTime HD Camera
# [AVFoundation indev @ 0x...] [1] Capture screen 0
# [AVFoundation indev @ 0x...] [2] Capture screen 1
```

### Option 3: Capture Specific Window

```bash
# Capture window by title
ffmpeg -f avfoundation -capture_cursor 1 -i "1:0" \
  -vf "crop=1920:1080:0:0" \
  -c:v libx264 -preset ultrafast \
  -f rtp_mpegts rtp://239.0.0.1:5004?ttl=1
```

---

## 🛠️ ĐỂ IMPLEMENT TEACHER MODE NATIVE

### Cần làm:

1. **Implement AVFoundation Capture**
   ```rust
   // src-tauri/src/screen_capture_macos.rs
   use core_graphics::display::*;
   use screencapturekit::*; // macOS 12.3+
   ```

2. **Request Screen Recording Permission**
   ```rust
   use core_graphics::display::CGRequestScreenCaptureAccess;
   
   if !CGRequestScreenCaptureAccess() {
       return Err("Screen recording permission denied");
   }
   ```

3. **Add Entitlements**
   ```xml
   <!-- src-tauri/Info.plist -->
   <key>NSCameraUsageDescription</key>
   <string>Screen recording for teaching</string>
   ```

4. **Capture Implementation**
   ```rust
   // Pseudo-code
   let display = CGMainDisplay();
   let image = CGDisplayCreateImage(display);
   // Convert to raw bytes
   // Send to FFmpeg encoder
   ```

---

## 📋 SETUP CHO macOS (STUDENT MODE)

### 1. Cài FFmpeg
```bash
brew install ffmpeg
```

### 2. Cài Dependencies
```bash
npm install
```

### 3. Chạy App
```bash
npm run tauri dev
```

### 4. Sử dụng Student Mode
- Chọn "Student Mode"
- Nhập multicast address: `239.0.0.1:5004`
- Click "Connect to Teacher"
- ✓ Xem stream từ Teacher (Windows)

---

## 🎯 USE CASE THỰC TẾ

### Phòng máy với macOS:

**Scenario 1: Teacher Windows, Students macOS**
- ✅ Teacher (Windows): Chạy app bình thường
- ✅ Students (macOS): Chạy Student mode
- ✅ Hoạt động hoàn hảo!

**Scenario 2: Teacher macOS, Students macOS**
- ❌ Teacher (macOS): Dùng FFmpeg command line
- ✅ Students (macOS): Chạy Student mode
- ⚠️ Teacher phải chạy manual command

**Scenario 3: All macOS + Native Support**
- ✅ Implement AVFoundation capture
- ✅ Teacher và Students đều dùng app
- ✅ Hoạt động như Windows

---

## 🚀 QUICK START (macOS - Student Only)

```bash
# 1. Cài FFmpeg
brew install ffmpeg

# 2. Cài dependencies
npm install

# 3. Chạy app
npm run tauri dev

# 4. Chọn Student Mode và connect!
```

---

## 📚 TÀI LIỆU THAM KHẢO

- [AVFoundation Screen Capture](https://developer.apple.com/documentation/avfoundation/avcapturesession)
- [ScreenCaptureKit (macOS 12.3+)](https://developer.apple.com/documentation/screencapturekit)
- [FFmpeg AVFoundation](https://ffmpeg.org/ffmpeg-devices.html#avfoundation)
- [Core Graphics Display](https://developer.apple.com/documentation/coregraphics/cgdisplay)

---

## ✅ CHECKLIST

### Student Mode (Hoạt động):
- [ ] Homebrew installed
- [ ] FFmpeg installed (`brew install ffmpeg`)
- [ ] Dependencies installed (`npm install`)
- [ ] App running (`npm run tauri dev`)
- [ ] Student mode works ✓

### Teacher Mode (Cần implement):
- [ ] AVFoundation capture implemented
- [ ] Screen recording permission
- [ ] Entitlements configured
- [ ] Build and test

---

## 💡 KHUYẾN NGHỊ

**Cho phòng máy thực tế:**
- Dùng **Windows cho Teacher machine** (DXGI hoạt động tốt)
- macOS/Windows cho Student machines (đều OK)
- Hoặc implement AVFoundation cho macOS Teacher

**Cho development:**
- Test Student mode trên macOS (hoạt động)
- Test Teacher mode trên Windows (hoạt động)
- Implement AVFoundation nếu cần macOS Teacher

---

Hiện tại app **hoạt động hoàn hảo cho Student mode trên macOS**! 🎉
