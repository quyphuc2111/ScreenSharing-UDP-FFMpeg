# 🚀 QUICK START - 5 PHÚT ĐỂ CHẠY

## BƯỚC 1: DOWNLOAD FFMPEG TỰ ĐỘNG (2 phút)

### Windows - PowerShell Script (Khuyến nghị)
```powershell
# Chạy script tự động download và setup FFmpeg
.\download_ffmpeg.ps1
```

Script sẽ:
- ✓ Download FFmpeg essentials (~150 MB)
- ✓ Extract và copy ffmpeg.exe, ffplay.exe
- ✓ Đặt vào `src-tauri/resources/`
- ✓ Cleanup temporary files

### Hoặc Manual Download
```powershell
# 1. Download: https://www.gyan.dev/ffmpeg/builds/ffmpeg-release-essentials.zip
# 2. Extract và copy:
mkdir src-tauri\resources
copy "ffmpeg-*\bin\ffmpeg.exe" "src-tauri\resources\"
copy "ffmpeg-*\bin\ffplay.exe" "src-tauri\resources\"
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

```powershell
# Download FFmpeg, cài dependencies và chạy
.\download_ffmpeg.ps1 ; npm install ; npm run tauri dev
```

---

## 🐛 NẾU CÓ LỖI

### "FFmpeg not found"
```powershell
# Chạy lại script download
.\download_ffmpeg.ps1

# Hoặc verify files
dir src-tauri\resources\
# Phải có: ffmpeg.exe và ffplay.exe
```

### "Cannot start streaming"
```powershell
# Chạy với Administrator
# Right-click app → Run as Administrator
```

### "Student không thấy stream"
```powershell
# Tắt firewall test
netsh advfirewall set allprofiles state off
```

---

## 📦 BUILD PRODUCTION (FFmpeg đã được bundle!)

```bash
npm run tauri build
```

File .exe trong: `src-tauri/target/release/`

**Lưu ý:** FFmpeg đã được tích hợp sẵn, không cần cài thêm!

---

## 🎯 DEFAULT SETTINGS

- **Multicast**: 239.0.0.1:5004
- **Bitrate**: 4 Mbps
- **FPS**: 30
- **Resolution**: Auto (màn hình hiện tại)
- **FFmpeg**: Bundled (không cần cài riêng)

---

## ✅ CHECKLIST

- [ ] FFmpeg downloaded (`.\download_ffmpeg.ps1`)
- [ ] Verify files exist (`dir src-tauri\resources\`)
- [ ] Dependencies installed (`npm install`)
- [ ] App running (`npm run tauri dev`)
- [ ] Teacher streaming
- [ ] Student viewing

**Tổng thời gian: ~5 phút** ⏱️

---

## 📊 KÍCH THƯỚC

- App (without FFmpeg): ~5 MB
- FFmpeg bundled: ~100 MB
- FFplay bundled: ~100 MB
- **Total installer: ~205 MB**

Nhưng chỉ cần deploy 1 lần, không cần cài FFmpeg riêng trên 50 máy!
