# HỆ THỐNG CHIA SẺ MÀN HÌNH PHÒNG MÁY - WINDOWS

Hệ thống chia sẻ màn hình hiệu năng cao cho phòng máy tính Windows, hỗ trợ 1 Teacher và 30-50 Students, hoạt động hoàn toàn offline trong mạng LAN.

## ✨ ĐẶC ĐIỂM NỔI BẬT

- **Windows Native**: Sử dụng DXGI Desktop Duplication API
- **FFmpeg Bundled**: Không cần cài đặt FFmpeg riêng
- **Hiệu năng cao**: Không giật lag, hình ảnh sắc nét, độ trễ ~100-150ms
- **Băng thông cố định**: Teacher chỉ tốn 5 Mbps bất kể số lượng Student
- **Scalable**: Hỗ trợ 30-50 Students đồng thời
- **Offline**: Hoạt động 100% trong LAN, không cần Internet

## 🚀 QUICK START (5 PHÚT)

### Bước 1: Download FFmpeg (2 phút)
```powershell
.\download_ffmpeg.ps1
```

### Bước 2: Cài Dependencies (1 phút)
```bash
npm install
```

### Bước 3: Build cho Windows (2 phút)
```bash
npm run tauri build
```

### Bước 4: Deploy
Copy installer từ `src-tauri/target/release/bundle/nsis/` vào USB và cài trên các máy Windows.

## 📦 DEPLOYMENT CHO PHÒNG MÁY

### 1. Build trên máy Dev (macOS/Windows)
```bash
# Download FFmpeg
.\download_ffmpeg.ps1

# Build
npm run tauri build
```

### 2. Lấy Installer
```
src-tauri/target/release/bundle/nsis/
└── screensharing-udp-ffmpeg_0.1.0_x64-setup.exe (~210 MB)
```

### 3. Cài trên 50 máy Windows
- Copy installer vào USB
- Chạy installer trên từng máy
- Không cần cài FFmpeg riêng!

## 🎯 SỬ DỤNG

### Máy Teacher:
1. Mở app
2. Chọn "Teacher Mode"
3. Click "Start Streaming"
4. ✓ Màn hình được chia sẻ!

### Máy Student (x50):
1. Mở app
2. Chọn "Student Mode"
3. Click "Connect to Teacher"
4. ✓ Xem màn hình Teacher!

## 🏗️ KIẾN TRÚC

```
Teacher (Windows) ──[UDP Multicast 5 Mbps]──> Switch ──> 50 Students (Windows)
```

### Công nghệ:
- **Capture**: DXGI Desktop Duplication (Windows)
- **Encode**: H.264 (FFmpeg libx264/NVENC)
- **Transport**: RTP over UDP Multicast
- **Decode**: GPU Hardware Decode (DXVA2)
- **Render**: DirectX

## 📋 YÊU CẦU HỆ THỐNG

### Teacher Machine:
- OS: Windows 10/11
- CPU: Intel i5 gen 6+ hoặc AMD Ryzen 5+
- RAM: 8GB+
- GPU: Bất kỳ (khuyến nghị NVIDIA/Intel cho GPU encoding)
- Network: Gigabit Ethernet

### Student Machine:
- OS: Windows 10/11
- CPU: Intel i3 gen 4+
- RAM: 4GB+
- GPU: Intel HD Graphics 4000+ (hỗ trợ DXVA2)
- Network: Fast Ethernet (100Mbps) hoặc Gigabit

### Network:
- Switch: Gigabit, hỗ trợ IGMP Snooping
- Subnet: Tất cả máy cùng subnet (VD: 192.168.1.x/24)

## 🎨 GIAO DIỆN

- Modern UI với gradient backgrounds
- Glassmorphism effects
- Smooth animations
- Dark theme
- Responsive design

## 📊 HIỆU NĂNG

### Teacher:
- CPU: ~15-25% (libx264) hoặc ~5% (NVENC)
- RAM: ~200-300 MB
- Network Upload: 5 Mbps (cố định)

### Student:
- CPU: ~5-10%
- GPU: ~10-15% (hardware decode)
- RAM: ~100-150 MB
- Network Download: 5 Mbps

### Độ trễ: ~100-150ms ✓

## 🔧 CẤU HÌNH FIREWALL

### Teacher:
```powershell
netsh advfirewall firewall add rule name="Screen Share Teacher" ^
  dir=out action=allow protocol=UDP remoteip=239.0.0.1 remoteport=5004
```

### Students:
```powershell
netsh advfirewall firewall add rule name="Screen Share Student" ^
  dir=in action=allow protocol=UDP localip=239.0.0.1 localport=5004
```

## 📚 TÀI LIỆU

- **QUICK_START.md** - Hướng dẫn nhanh
- **DEPLOYMENT_GUIDE.md** - Hướng dẫn triển khai chi tiết
- **SYSTEM_DESIGN.md** - Thiết kế kỹ thuật
- **ARCHITECTURE_DIAGRAM.md** - Sơ đồ kiến trúc
- **BUNDLE_FFMPEG_GUIDE.md** - Hướng dẫn bundle FFmpeg

## 🐛 TROUBLESHOOTING

### "FFmpeg not found"
```powershell
.\download_ffmpeg.ps1
```

### "Cannot start streaming"
- Chạy app với quyền Administrator
- Check firewall rules

### "Student không thấy stream"
- Verify cùng subnet: `ipconfig`
- Test ping Teacher
- Check firewall

### Giật lag
- Giảm bitrate xuống 3M
- Giảm resolution
- Check network cable

## 🎯 DEFAULT SETTINGS

- **Multicast**: 239.0.0.1:5004
- **Bitrate**: 4 Mbps
- **FPS**: 30
- **Resolution**: Auto (màn hình hiện tại)
- **FFmpeg**: Bundled (không cần cài riêng)

## 📦 KÍCH THƯỚC

- App: ~5 MB
- FFmpeg bundled: ~100 MB
- FFplay bundled: ~100 MB
- **Total installer: ~210 MB**

Deploy 1 lần, không cần cài FFmpeg trên 50 máy!

## ✅ CHECKLIST DEPLOYMENT

- [ ] Download FFmpeg (`.\download_ffmpeg.ps1`)
- [ ] Build app (`npm run tauri build`)
- [ ] Test installer trên 1 máy Windows
- [ ] Copy installer vào USB
- [ ] Cài trên 50 máy
- [ ] Cấu hình firewall
- [ ] Test Teacher streaming
- [ ] Test Students viewing

## 🤝 SUPPORT

Nếu có vấn đề, check:
1. TROUBLESHOOTING section
2. DEPLOYMENT_GUIDE.md
3. Console logs trong app

---

**Lưu ý**: App được tối ưu cho Windows. Trên macOS chỉ hỗ trợ Student mode (xem stream).
