# HƯỚNG DẪN BUNDLE FFMPEG VÀO ỨNG DỤNG

## 🎯 MỤC ĐÍCH

Tích hợp FFmpeg trực tiếp vào ứng dụng để không cần cài đặt riêng.

## 📦 CÁCH 1: BUNDLE FFMPEG BINARIES (KHUYẾN NGHỊ)

### Bước 1: Download FFmpeg

**Windows:**
```powershell
# Download FFmpeg essentials build
# Link: https://www.gyan.dev/ffmpeg/builds/ffmpeg-release-essentials.zip

# Hoặc dùng PowerShell:
Invoke-WebRequest -Uri "https://www.gyan.dev/ffmpeg/builds/ffmpeg-release-essentials.zip" -OutFile "ffmpeg.zip"
Expand-Archive -Path "ffmpeg.zip" -DestinationPath "."
```

### Bước 2: Copy binaries vào project

```powershell
# Tạo thư mục resources
mkdir src-tauri\resources

# Copy ffmpeg.exe và ffplay.exe
copy "ffmpeg-*\bin\ffmpeg.exe" "src-tauri\resources\"
copy "ffmpeg-*\bin\ffplay.exe" "src-tauri\resources\"
```

### Bước 3: Verify

```
src-tauri/
├── resources/
│   ├── ffmpeg.exe  (✓ ~100 MB)
│   └── ffplay.exe  (✓ ~100 MB)
└── ...
```

### Bước 4: Build

```bash
npm run tauri build
```

FFmpeg sẽ được bundle vào file .exe hoặc installer!

## 📂 CẤU TRÚC SAU KHI BUILD

```
target/release/
├── screensharing-udp-ffmpeg.exe  (Main app)
├── ffmpeg.exe                     (Bundled)
└── ffplay.exe                     (Bundled)
```

Hoặc trong installer:
```
C:\Program Files\ScreenSharing\
├── screensharing-udp-ffmpeg.exe
├── resources/
│   ├── ffmpeg.exe
│   └── ffplay.exe
└── ...
```

## 🔍 CÁCH ỨNG DỤNG TÌM FFMPEG

Code đã được update để tìm FFmpeg theo thứ tự:

1. **Bundled binary** (cùng thư mục với .exe)
   - `./ffmpeg.exe`
   - `./ffplay.exe`

2. **Resources folder**
   - `./resources/ffmpeg.exe`
   - `./resources/ffplay.exe`

3. **System PATH** (fallback)
   - `ffmpeg` (nếu đã cài)
   - `ffplay` (nếu đã cài)

## ⚙️ CÁCH 2: GIẢM KÍCH THƯỚC (OPTIONAL)

FFmpeg essentials build ~100MB mỗi file. Để giảm:

### Option A: Sử dụng FFmpeg Static Build (Nhỏ hơn)

```powershell
# Download static build (smaller)
# Link: https://github.com/BtbN/FFmpeg-Builds/releases
# File: ffmpeg-master-latest-win64-gpl-shared.zip (~50MB)
```

### Option B: Custom Build (Chỉ cần H.264)

Build FFmpeg với chỉ libx264:
```bash
./configure --enable-gpl --enable-libx264 --disable-everything --enable-encoder=libx264 --enable-decoder=h264 --enable-muxer=rtp --enable-protocol=rtp,udp
make
```

Kích thước: ~20-30MB

## 🚀 DEPLOYMENT

### Development (với bundled FFmpeg)

```bash
# FFmpeg đã có trong resources/
npm run tauri dev
```

### Production Build

```bash
npm run tauri build
```

Output:
- **Installer**: `src-tauri/target/release/bundle/nsis/screensharing-udp-ffmpeg_0.1.0_x64-setup.exe`
- **Portable**: `src-tauri/target/release/screensharing-udp-ffmpeg.exe` + `ffmpeg.exe` + `ffplay.exe`

## 📋 CHECKLIST

- [ ] Download FFmpeg essentials
- [ ] Copy ffmpeg.exe vào `src-tauri/resources/`
- [ ] Copy ffplay.exe vào `src-tauri/resources/`
- [ ] Verify files tồn tại
- [ ] Build: `npm run tauri build`
- [ ] Test installer
- [ ] Verify app chạy không cần cài FFmpeg

## 🐛 TROUBLESHOOTING

### "FFmpeg not found" sau khi bundle

**Nguyên nhân:** Files không được copy đúng

**Giải pháp:**
```powershell
# Verify files tồn tại
dir src-tauri\resources\

# Nếu không có, copy lại
copy "path\to\ffmpeg.exe" "src-tauri\resources\"
copy "path\to\ffplay.exe" "src-tauri\resources\"
```

### Kích thước installer quá lớn

**Nguyên nhân:** FFmpeg essentials ~200MB

**Giải pháp:**
1. Dùng FFmpeg static build (nhỏ hơn)
2. Hoặc custom build chỉ H.264
3. Hoặc compress với UPX:
```powershell
upx --best ffmpeg.exe
upx --best ffplay.exe
```

### App không tìm thấy FFmpeg khi chạy

**Nguyên nhân:** Path không đúng

**Giải pháp:**
- Check console log để xem path nào được thử
- Verify `tauri.conf.json` có `resources` và `externalBin`
- Rebuild: `npm run tauri build`

## 📊 KÍCH THƯỚC THAM KHẢO

| Component | Size |
|-----------|------|
| App (without FFmpeg) | ~5 MB |
| FFmpeg essentials | ~100 MB |
| FFplay essentials | ~100 MB |
| **Total** | **~205 MB** |

Với compression (UPX):
| Component | Size |
|-----------|------|
| App | ~5 MB |
| FFmpeg (compressed) | ~40 MB |
| FFplay (compressed) | ~40 MB |
| **Total** | **~85 MB** |

## 🎁 ALTERNATIVE: DOWNLOAD ON FIRST RUN

Nếu không muốn bundle (giảm kích thước installer):

1. App kiểm tra FFmpeg khi khởi động
2. Nếu không có, hiển thị dialog download
3. Download FFmpeg từ CDN
4. Extract vào app folder
5. Sử dụng

Code mẫu đã có trong `streaming.rs` - chỉ cần thêm download logic!

## ✅ RECOMMENDED APPROACH

**Cho phòng máy (30-50 máy):**
- Bundle FFmpeg vào installer
- Deploy 1 lần, chạy mãi mãi
- Không cần Internet
- Không cần cài đặt thêm

**Cho distribution rộng:**
- Download on first run
- Installer nhỏ (~5MB)
- Tự động download FFmpeg khi cần
- Tiết kiệm bandwidth

---

**Lưu ý:** FFmpeg là GPL license. Nếu distribute app, cần tuân thủ GPL hoặc mua commercial license.
