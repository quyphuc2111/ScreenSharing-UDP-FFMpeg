# 🚀 QUICK START - macOS

## BƯỚC 1: CÀI FFMPEG (2 phút)

### Option 1: Homebrew (Khuyến nghị)
```bash
# Cài FFmpeg qua Homebrew
brew install ffmpeg

# Verify
ffmpeg -version
```

### Option 2: Auto-install Script
```bash
# Chạy script tự động
chmod +x download_ffmpeg.sh
./download_ffmpeg.sh
```

## BƯỚC 2: CÀI DEPENDENCIES (1 phút)

```bash
npm install
```

## BƯỚC 3: CHẠY APP (1 phút)

```bash
npm run tauri dev
```

## BƯỚC 4: SỬ DỤNG (1 phút)

### Máy Teacher:
1. Chọn "Teacher Mode"
2. Click "Start Streaming"
3. ✓ Done!

### Máy Student:
1. Chọn "Student Mode"
2. Click "Connect to Teacher"
3. ✓ Xem màn hình Teacher!

---

## ⚡ ONE-LINER SETUP

```bash
# Cài FFmpeg, dependencies và chạy
brew install ffmpeg && npm install && npm run tauri dev
```

---

## 🐛 NẾU CÓ LỖI

### "FFmpeg not found"
```bash
# Cài FFmpeg
brew install ffmpeg

# Verify
which ffmpeg
ffmpeg -version
```

### "Screen capture not supported"
```
Lưu ý: DXGI Desktop Duplication chỉ có trên Windows.
Trên macOS, cần implement AVFoundation screen capture.
```

### "Student không thấy stream"
```bash
# Check firewall
sudo /usr/libexec/ApplicationFirewall/socketfilterfw --getglobalstate

# Tắt firewall (test only)
sudo /usr/libexec/ApplicationFirewall/socketfilterfw --setglobalstate off
```

---

## 📦 BUILD PRODUCTION

```bash
npm run tauri build
```

Output: `src-tauri/target/release/bundle/macos/`

---

## 🎯 DEFAULT SETTINGS

- **Multicast**: 239.0.0.1:5004
- **Bitrate**: 4 Mbps
- **FPS**: 30
- **Resolution**: Auto
- **FFmpeg**: System (via Homebrew)

---

## ⚠️ LƯU Ý QUAN TRỌNG

### Screen Capture trên macOS

App hiện tại sử dụng DXGI (Windows only). Để chạy trên macOS cần:

1. **Implement AVFoundation capture** (thay DXGI)
2. **Screen Recording Permission** (macOS 10.15+)
3. **Entitlements** trong Tauri config

### Workaround hiện tại:

Dùng FFmpeg để capture màn hình macOS:

```bash
# Teacher mode (manual)
ffmpeg -f avfoundation -i "1:0" \
  -c:v libx264 -preset ultrafast -tune zerolatency \
  -profile:v baseline -pix_fmt yuv420p \
  -g 60 -b:v 4M -maxrate 5M -bufsize 2M \
  -f rtp_mpegts rtp://239.0.0.1:5004?ttl=1

# Student mode
ffplay rtp://239.0.0.1:5004
```

---

## ✅ CHECKLIST

- [ ] Homebrew installed
- [ ] FFmpeg installed (`brew install ffmpeg`)
- [ ] Dependencies installed (`npm install`)
- [ ] App running (`npm run tauri dev`)
- [ ] (Windows only) Teacher streaming
- [ ] Student viewing

---

## 🔧 DEVELOPMENT NOTES

### Để hỗ trợ macOS đầy đủ:

1. **Replace DXGI với AVFoundation**
   ```rust
   // src-tauri/src/screen_capture_macos.rs
   use core_graphics::display::*;
   use core_foundation::*;
   ```

2. **Add entitlements**
   ```xml
   <!-- src-tauri/Info.plist -->
   <key>NSCameraUsageDescription</key>
   <string>Screen recording for teaching</string>
   ```

3. **Request permissions**
   ```rust
   // Request screen recording permission
   CGRequestScreenCaptureAccess();
   ```

---

## 📚 TÀI LIỆU THAM KHẢO

- [AVFoundation Screen Capture](https://developer.apple.com/documentation/avfoundation/avcapturesession)
- [macOS Screen Recording Permission](https://developer.apple.com/documentation/avfoundation/cameras_and_media_capture/requesting_authorization_for_media_capture_on_macos)
- [FFmpeg AVFoundation](https://ffmpeg.org/ffmpeg-devices.html#avfoundation)

---

**Tổng thời gian: ~5 phút** ⏱️
