# HƯỚNG DẪN TRIỂN KHAI PHÒNG MÁY

## BƯỚC 1: CHUẨN BỊ HẠ TẦNG

### 1.1 Yêu cầu phần cứng

**Máy Teacher:**
- CPU: Intel i5 gen 6+ hoặc AMD Ryzen 5+
- RAM: 8GB+
- GPU: Bất kỳ (Intel HD Graphics, NVIDIA, AMD)
- Network: Gigabit Ethernet (khuyến nghị)

**Máy Student:**
- CPU: Intel i3 gen 4+ hoặc tương đương
- RAM: 4GB+
- GPU: Intel HD Graphics 4000+ hoặc tương đương
- Network: Gigabit Ethernet hoặc Fast Ethernet (100Mbps)

**Switch:**
- Gigabit Switch (khuyến nghị)
- Hỗ trợ IGMP Snooping
- Managed hoặc Smart Switch (tốt hơn)

### 1.2 Cấu hình mạng

**Subnet:** 192.168.1.0/24 (hoặc bất kỳ subnet nào)
- Teacher: 192.168.1.10
- Students: 192.168.1.11 - 192.168.1.60

**Multicast:**
- IP: 239.0.0.1
- Port: 5004
- TTL: 1

### 1.3 Cấu hình Switch

```
1. Enable IGMP Snooping
2. Enable Multicast Filtering (nếu có)
3. Set IGMP version: v2 hoặc v3
4. Disable Multicast Storm Control (nếu bị block)
```

## BƯỚC 2: CÀI ĐẶT PHẦN MỀM

### 2.1 Cài FFmpeg (Tất cả máy)

**Windows:**
```powershell
# Download từ: https://ffmpeg.org/download.html
# Hoặc dùng Chocolatey:
choco install ffmpeg

# Verify
ffmpeg -version
```

**Thêm vào PATH:**
```
1. Giải nén ffmpeg.zip
2. Copy folder vào C:\ffmpeg
3. Thêm C:\ffmpeg\bin vào System PATH
4. Restart Command Prompt
```

### 2.2 Cài GPU Drivers (Quan trọng!)

**Intel:**
- Download Intel Graphics Driver mới nhất
- Đảm bảo hỗ trợ QuickSync

**NVIDIA:**
- Download GeForce/Quadro Driver mới nhất
- Verify NVENC support: `ffmpeg -encoders | grep nvenc`

**AMD:**
- Download AMD Driver mới nhất
- Verify VCE/VCN support

## BƯỚC 3: CẤU HÌNH FIREWALL

### 3.1 Windows Firewall (Tất cả máy)

**Teacher:**
```powershell
netsh advfirewall firewall add rule name="Screen Share Teacher Out" ^
  dir=out action=allow protocol=UDP remoteip=239.0.0.1 remoteport=5004

netsh advfirewall firewall add rule name="Screen Share Teacher In" ^
  dir=in action=allow protocol=UDP localip=239.0.0.1 localport=5004
```

**Students:**
```powershell
netsh advfirewall firewall add rule name="Screen Share Student" ^
  dir=in action=allow protocol=UDP localip=239.0.0.1 localport=5004
```

### 3.2 Tắt Firewall (Nếu cần thiết)
```powershell
# Chỉ trong môi trường LAN an toàn
netsh advfirewall set allprofiles state off
```

## BƯỚC 4: TESTING

### 4.1 Test Multicast Connectivity

**Teacher (Gửi test stream):**
```batch
ffmpeg -f lavfi -i testsrc=size=1280x720:rate=30 ^
  -c:v libx264 -preset ultrafast -tune zerolatency ^
  -b:v 2M -f rtp_mpegts rtp://239.0.0.1:5004?ttl=1
```

**Student (Nhận test stream):**
```batch
ffplay rtp://239.0.0.1:5004
```

**Kết quả mong đợi:**
- Student thấy màn hình test pattern (màu sắc chuyển động)
- Không giật lag
- Độ trễ < 500ms

### 4.2 Test với 1 Student
1. Teacher chạy screen capture
2. 1 Student join
3. Verify: Hình ảnh rõ, không lag, độ trễ thấp

### 4.3 Test với 10 Students
1. Teacher chạy screen capture
2. 10 Students join đồng thời
3. Monitor Teacher CPU/Network
4. Verify: Băng thông không tăng, CPU ổn định

### 4.4 Test với 50 Students (Full load)
1. Teacher chạy screen capture
2. 50 Students join dần dần
3. Monitor: CPU, RAM, Network, Packet loss
4. Verify: Tất cả Students nhận stream ổn định

## BƯỚC 5: DEPLOYMENT SCRIPTS

### 5.1 Teacher Script (teacher_start.bat)
```batch
@echo off
title Teacher - Screen Share Server
color 0A

echo ========================================
echo   TEACHER SCREEN SHARE SERVER
echo ========================================
echo.
echo Multicast: 239.0.0.1:5004
echo Resolution: 1920x1080 @ 30fps
echo Bitrate: 4-5 Mbps
echo.
echo Starting in 3 seconds...
timeout /t 3 /nobreak >nul

echo.
echo [STREAMING] Press Ctrl+C to stop
echo ========================================
echo.

REM Chọn encoder phù hợp
REM Option 1: CPU encoding (libx264)
ffmpeg -f gdigrab -framerate 30 -i desktop ^
  -c:v libx264 -preset ultrafast -tune zerolatency ^
  -profile:v baseline -level 3.1 -pix_fmt yuv420p ^
  -g 60 -keyint_min 60 ^
  -b:v 4M -maxrate 5M -bufsize 2M ^
  -f rtp_mpegts rtp://239.0.0.1:5004?ttl=1

REM Option 2: NVIDIA GPU encoding (uncomment nếu có NVIDIA GPU)
REM ffmpeg -f gdigrab -framerate 30 -i desktop ^
REM   -c:v h264_nvenc -preset p1 -tune ll ^
REM   -profile:v baseline -rc vbr ^
REM   -b:v 4M -maxrate 5M -g 60 ^
REM   -f rtp_mpegts rtp://239.0.0.1:5004?ttl=1

REM Option 3: Intel QuickSync (uncomment nếu có Intel GPU)
REM ffmpeg -f gdigrab -framerate 30 -i desktop ^
REM   -c:v h264_qsv -preset veryfast ^
REM   -profile:v baseline -b:v 4M -maxrate 5M -g 60 ^
REM   -f rtp_mpegts rtp://239.0.0.1:5004?ttl=1

echo.
echo [STOPPED] Streaming stopped.
pause
```

### 5.2 Student Script (student_start.bat)
```batch
@echo off
title Student - Screen Share Client
color 0B

echo ========================================
echo   STUDENT SCREEN SHARE CLIENT
echo ========================================
echo.
echo Connecting to: 239.0.0.1:5004
echo Teacher: %TEACHER_IP%
echo.
echo Starting in 2 seconds...
timeout /t 2 /nobreak >nul

echo.
echo [CONNECTED] Press Q to quit
echo ========================================
echo.

ffplay -fflags nobuffer -flags low_delay -framedrop ^
  -probesize 32 -analyzeduration 0 ^
  -hwaccel auto ^
  -fs ^
  -window_title "Teacher Screen" ^
  rtp://239.0.0.1:5004

echo.
echo [DISCONNECTED] Connection closed.
pause
```

## BƯỚC 6: MONITORING & TROUBLESHOOTING

### 6.1 Monitor Teacher Performance

**Task Manager:**
- CPU: Không vượt quá 30%
- Network: ~5 Mbps (cố định)
- RAM: ~300 MB

**FFmpeg Stats:**
```batch
REM Thêm vào command để xem stats
-progress pipe:1 2>&1 | findstr "fps bitrate"
```

### 6.2 Monitor Student Performance

**Task Manager:**
- CPU: < 15%
- GPU: 10-20%
- Network: ~5 Mbps download

### 6.3 Common Issues

**Issue 1: Students không nhận được stream**
```
Nguyên nhân:
- Firewall block
- Switch không hỗ trợ multicast
- Không cùng subnet

Giải pháp:
1. Tắt firewall test
2. Check switch IGMP settings
3. Verify IP subnet: ipconfig
4. Test ping giữa Teacher và Student
```

**Issue 2: Giật lag**
```
Nguyên nhân:
- Network congestion
- Packet loss cao
- CPU/GPU quá tải

Giải pháp:
1. Giảm bitrate: -b:v 3M
2. Giảm resolution: -vf scale=1280:720
3. Giảm FPS: -framerate 25
4. Check network cable
```

**Issue 3: Độ trễ cao (> 1 giây)**
```
Nguyên nhân:
- Buffer quá lớn
- Decode chậm
- Network latency

Giải pháp:
1. Giảm buffer: -bufsize 1M
2. Tăng GOP: -g 30
3. Dùng GPU decode: -hwaccel auto
```

**Issue 4: CPU Teacher quá cao**
```
Giải pháp:
1. Dùng GPU encoder (NVENC/QuickSync)
2. Giảm preset: ultrafast
3. Giảm resolution
4. Giảm FPS
```

## BƯỚC 7: PRODUCTION CHECKLIST

- [ ] Tất cả máy đã cài FFmpeg
- [ ] GPU drivers updated
- [ ] Firewall configured
- [ ] Switch IGMP enabled
- [ ] Test với 1 student thành công
- [ ] Test với 10 students thành công
- [ ] Test với 50 students thành công
- [ ] Teacher script ready
- [ ] Student script ready
- [ ] Backup plan (nếu multicast fail)
- [ ] User training completed
