# HỆ THỐNG CHIA SẺ MÀN HÌNH PHÒNG MÁY - THIẾT KẾ KỸ THUẬT

## 1. TỔNG QUAN KIẾN TRÚC

### Mô hình hoạt động
- **1 Teacher** → Capture + Encode + Multicast
- **30-50 Students** → Join Multicast + Decode + Render
- **Giao thức**: UDP Multicast (239.0.0.1:5004)
- **Codec**: H.264 over RTP
- **Băng thông**: ~5 Mbps (cố định, không phụ thuộc số lượng client)

### Ưu điểm kiến trúc này
- Teacher chỉ encode 1 lần duy nhất
- Băng thông không tăng theo số lượng Student
- Student join/leave không ảnh hưởng Teacher
- Độ trễ thấp (~100-300ms)
- Không cần server trung gian

## 2. TEACHER - CAPTURE & STREAMING

### 2.1 DXGI Desktop Duplication
```
Windows API: DXGI 1.2+
- IDXGIOutputDuplication::AcquireNextFrame()
- Capture toàn màn hình hoặc monitor cụ thể
- Output: BGRA texture (GPU memory)
- FPS: 25-30 (configurable)
```

### 2.2 FFmpeg Encoding Pipeline
```bash
# Command line example (for testing)
ffmpeg -f gdigrab -framerate 30 -i desktop \
  -c:v libx264 \
  -preset ultrafast \
  -tune zerolatency \
  -profile:v baseline \
  -level 3.1 \
  -pix_fmt yuv420p \
  -g 60 \
  -keyint_min 60 \
  -b:v 4M \
  -maxrate 5M \
  -bufsize 2M \
  -f rtp_mpegts \
  rtp://239.0.0.1:5004
```

### 2.3 Thông số encoding quan trọng
- **Preset**: `ultrafast` (độ trễ thấp nhất) hoặc `veryfast` (chất lượng tốt hơn)
- **Profile**: `baseline` (tương thích GPU decode tốt nhất)
- **GOP**: 60 frames (2 giây @ 30fps) - keyframe interval
- **Bitrate**: 4-5 Mbps cho 1920x1080, 3 Mbps cho 1280x720
- **Pixel format**: yuv420p (chuẩn cho H.264)

## 3. NETWORK - UDP MULTICAST

### 3.1 Multicast Configuration
```
Multicast IP: 239.0.0.1 (Class D, LAN-safe)
Port: 5004
TTL: 1 (chỉ trong LAN, không route ra ngoài)
Protocol: RTP over UDP
```

### 3.2 Switch Requirements
- Switch phải hỗ trợ IGMP (Internet Group Management Protocol)
- Enable IGMP Snooping để tối ưu băng thông
- Không cần cấu hình phức tạp, hầu hết switch hiện đại đều hỗ trợ

### 3.3 Firewall Rules
```powershell
# Teacher machine
netsh advfirewall firewall add rule name="Screen Share Teacher" ^
  dir=out action=allow protocol=UDP remoteip=239.0.0.1 remoteport=5004

# Student machines
netsh advfirewall firewall add rule name="Screen Share Student" ^
  dir=in action=allow protocol=UDP localip=239.0.0.1 localport=5004
```

## 4. STUDENT - RECEIVE & DECODE

### 4.1 Join Multicast Group
```rust
// Socket join multicast
let socket = UdpSocket::bind("0.0.0.0:5004")?;
socket.join_multicast_v4(
    &"239.0.0.1".parse()?,
    &Ipv4Addr::UNSPECIFIED
)?;
```

### 4.2 GPU Decoding
- **Windows**: DXVA2 hoặc D3D11VA
- **Fallback**: Software decode (libavcodec)
- **Render**: DirectX 11/12 hoặc wgpu

### 4.3 FFmpeg Receive Command (Testing)
```bash
ffplay -fflags nobuffer -flags low_delay -framedrop \
  -probesize 32 -analyzeduration 0 \
  rtp://239.0.0.1:5004
```

## 5. HIỆU NĂNG & TỐI ƯU

### 5.1 Teacher Machine
- **CPU**: Encode H.264 ~15-25% (1 core, Intel i5+)
- **GPU**: DXGI capture ~5% (minimal)
- **RAM**: ~200-300 MB
- **Network**: 5 Mbps upload (cố định)

### 5.2 Student Machine
- **CPU**: ~5-10% (chỉ RTP processing)
- **GPU**: Decode ~10-15% (Intel HD Graphics trở lên)
- **RAM**: ~100-150 MB
- **Network**: 5 Mbps download

### 5.3 Độ trễ (Latency)
- Capture: ~16ms (1 frame @ 60fps)
- Encode: ~20-40ms (ultrafast preset)
- Network: ~5-10ms (LAN)
- Decode: ~20-30ms (GPU)
- **Tổng**: ~100-150ms (chấp nhận được)

## 6. XỬ LÝ MẤT GÓI (Packet Loss)

### 6.1 Chiến lược
- **Không retry**: UDP không đảm bảo delivery
- **Keyframe thường xuyên**: Mỗi 2 giây có 1 keyframe
- **Error concealment**: Decoder tự động xử lý frame bị lỗi
- **Acceptable loss**: 1-2% packet loss vẫn xem được

### 6.2 Giảm thiểu packet loss
- Sử dụng switch Gigabit
- Tránh WiFi cho Teacher machine
- QoS (Quality of Service) trên switch nếu có

## 7. TRIỂN KHAI THỰC TẾ

### 7.1 Chuẩn bị hạ tầng
1. Kiểm tra switch hỗ trợ multicast
2. Cấu hình IGMP Snooping
3. Đảm bảo tất cả máy cùng subnet (VD: 192.168.1.x/24)
4. Tắt firewall hoặc mở port 5004 UDP

### 7.2 Testing workflow
1. Chạy Teacher app → Bắt đầu streaming
2. Chạy 1 Student app → Verify nhận được stream
3. Chạy thêm 5-10 Students → Kiểm tra hiệu năng
4. Chạy full 50 Students → Stress test

### 7.3 Troubleshooting
- **Không nhận được stream**: Kiểm tra firewall, subnet, multicast support
- **Giật lag**: Giảm bitrate, giảm resolution, check network congestion
- **CPU cao**: Dùng GPU encode (NVENC/QuickSync) thay vì libx264
- **Màu sai**: Kiểm tra pixel format conversion (BGRA → YUV420p)

## 8. NÂNG CAO (OPTIONAL)

### 8.1 GPU Encoding (Teacher)
- **NVIDIA**: NVENC (H.264 hardware encoder)
- **Intel**: QuickSync (Intel HD Graphics 4000+)
- **AMD**: VCE/VCN
- Giảm CPU usage xuống ~5%, tăng chất lượng

### 8.2 Adaptive Bitrate
- Monitor network conditions
- Tự động giảm bitrate khi phát hiện packet loss cao
- Tăng bitrate khi network ổn định

### 8.3 Audio Streaming
- Capture audio bằng WASAPI (Windows)
- Encode AAC, mux với H.264
- Sync audio/video bằng PTS (Presentation Timestamp)

### 8.4 Control Channel
- Thêm TCP channel riêng cho control (Teacher ↔ Students)
- Gửi lệnh: pause, resume, change quality
- Nhận feedback: student count, network stats

## 9. SO SÁNH VỚI CÁC GIẢI PHÁP KHÁC

| Giải pháp | Băng thông | Độ trễ | Chất lượng | Phức tạp |
|-----------|------------|--------|------------|----------|
| **UDP Multicast H.264** | 5 Mbps (cố định) | 100-150ms | Cao | Trung bình |
| VNC/RDP | 50-500 Mbps | 200-500ms | Thấp | Thấp |
| WebRTC | 150-500 Mbps | 100-300ms | Cao | Cao |
| JPEG over WebSocket | 100-300 Mbps | 150-300ms | Trung bình | Thấp |

**Kết luận**: UDP Multicast H.264 là giải pháp tối ưu nhất cho phòng máy LAN.
