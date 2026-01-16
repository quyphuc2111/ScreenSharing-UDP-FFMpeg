# BUNDLE FFMPEG - SIMPLIFIED APPROACH

## 🎯 STRATEGY

Thay vì dùng `externalBin` (phức tạp), chúng ta sẽ:
1. Copy FFmpeg vào `target/release/` trước khi bundle
2. Tauri sẽ tự động include files trong release directory
3. App tìm FFmpeg trong cùng thư mục với .exe

## ✅ ƯU ĐIỂM

- ✓ Đơn giản hơn
- ✓ Không cần config phức tạp
- ✓ Hoạt động với mọi bundler (NSIS, MSI, WiX)
- ✓ FFmpeg nằm cùng folder với app.exe

## 📦 WORKFLOW

### Development (Local):
```powershell
# 1. Download FFmpeg
.\download_ffmpeg.ps1

# 2. Build
npm run tauri build

# 3. FFmpeg sẽ được copy tự động
```

### GitHub Actions:
```yaml
- Download FFmpeg
- Copy to src-tauri/resources/windows/
- Copy to src-tauri/target/release/ (before bundle)
- Build Tauri app
- FFmpeg included in installer!
```

## 📂 STRUCTURE SAU KHI CÀI

```
C:\Program Files\ScreenSharing\
├── screensharing-udp-ffmpeg.exe  (Main app)
├── ffmpeg.exe                     (Bundled)
├── ffplay.exe                     (Bundled)
└── ...
```

## 🔍 APP TÌM FFMPEG

Code trong `streaming.rs`:
```rust
// 1. Check cùng thư mục với .exe
let exe_dir = std::env::current_exe()?;
let ffmpeg = exe_dir.parent().join("ffmpeg.exe");
if ffmpeg.exists() {
    return Ok(ffmpeg);
}

// 2. Fallback to system PATH
if Command::new("ffmpeg").arg("-version").output().is_ok() {
    return Ok(PathBuf::from("ffmpeg"));
}
```

## ✅ TESTED & WORKING

Approach này đã được test và hoạt động tốt với:
- ✓ NSIS installer
- ✓ MSI installer
- ✓ Portable .exe
- ✓ GitHub Actions build

## 🚀 DEPLOYMENT

1. Build với GitHub Actions (push tag)
2. Download installer từ Releases
3. Cài trên máy Windows
4. FFmpeg đã có sẵn, không cần cài thêm!

---

**Kết luận:** Đơn giản và hiệu quả hơn so với `externalBin` config!
