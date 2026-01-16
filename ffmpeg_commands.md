# FFMPEG COMMANDS - TESTING & DEPLOYMENT

## 1. TEACHER - STREAMING COMMANDS

### 1.1 Basic Streaming (Screen Capture)
```bash
# Windows - GDI Grab (Simple, CPU-based)
ffmpeg -f gdigrab -framerate 30 -i desktop \
  -c:v libx264 -preset ultrafast -tune zerolatency \
  -profile:v baseline -level 3.1 -pix_fmt yuv420p \
  -g 60 -keyint_min 60 \
  -b:v 4M -maxrate 5M -bufsize 2M \
  -f rtp_mpegts rtp://239.0.0.1:5004?ttl=1
```

### 1.2 GPU Encoding - NVIDIA (NVENC)
```bash
ffmpeg -f gdigrab -framerate 30 -i desktop \
  -c:v h264_nvenc \
  -preset p1 \
  -tune ll \
  -profile:v baseline \
  -rc vbr \
  -b:v 4M -maxrate 5M \
  -g 60 \
  -f rtp_mpegts rtp://239.0.0.1:5004?ttl=1
```

### 1.3 GPU Encoding - Intel QuickSync
```bash
ffmpeg -f gdigrab -framerate 30 -i desktop \
  -c:v h264_qsv \
  -preset veryfast \
  -profile:v baseline \
  -b:v 4M -maxrate 5M \
  -g 60 \
  -f rtp_mpegts rtp://239.0.0.1:5004?ttl=1
```

### 1.4 Lower Resolution (1280x720)
```bash
ffmpeg -f gdigrab -framerate 30 -i desktop \
  -vf scale=1280:720 \
  -c:v libx264 -preset ultrafast -tune zerolatency \
  -profile:v baseline -pix_fmt yuv420p \
  -g 60 -b:v 3M -maxrate 4M -bufsize 1.5M \
  -f rtp_mpegts rtp://239.0.0.1:5004?ttl=1
```

### 1.5 Specific Monitor (Multi-monitor setup)
```bash
# Monitor 1 (primary)
ffmpeg -f gdigrab -framerate 30 -offset_x 0 -offset_y 0 -video_size 1920x1080 -i desktop \
  -c:v libx264 -preset ultrafast -tune zerolatency \
  -profile:v baseline -pix_fmt yuv420p \
  -g 60 -b:v 4M -maxrate 5M -bufsize 2M \
  -f rtp_mpegts rtp://239.0.0.1:5004?ttl=1

# Monitor 2 (secondary, offset 1920px)
ffmpeg -f gdigrab -framerate 30 -offset_x 1920 -offset_y 0 -video_size 1920x1080 -i desktop \
  -c:v libx264 -preset ultrafast -tune zerolatency \
  -profile:v baseline -pix_fmt yuv420p \
  -g 60 -b:v 4M -maxrate 5M -bufsize 2M \
  -f rtp_mpegts rtp://239.0.0.1:5004?ttl=1
```

## 2. STUDENT - RECEIVING COMMANDS

### 2.1 Basic Playback (FFplay)
```bash
ffplay -fflags nobuffer -flags low_delay -framedrop \
  -probesize 32 -analyzeduration 0 \
  rtp://239.0.0.1:5004
```

### 2.2 GPU Decoding - Auto
```bash
ffplay -fflags nobuffer -flags low_delay -framedrop \
  -probesize 32 -analyzeduration 0 \
  -hwaccel auto \
  rtp://239.0.0.1:5004
```

### 2.3 GPU Decoding - DXVA2 (Windows)
```bash
ffplay -fflags nobuffer -flags low_delay -framedrop \
  -probesize 32 -analyzeduration 0 \
  -hwaccel dxva2 \
  rtp://239.0.0.1:5004
```

### 2.4 GPU Decoding - NVIDIA CUVID
```bash
ffplay -fflags nobuffer -flags low_delay -framedrop \
  -probesize 32 -analyzeduration 0 \
  -c:v h264_cuvid \
  rtp://239.0.0.1:5004
```

### 2.5 Fullscreen Mode
```bash
ffplay -fflags nobuffer -flags low_delay -framedrop \
  -probesize 32 -analyzeduration 0 \
  -fs \
  rtp://239.0.0.1:5004
```

### 2.6 Save to File (Recording)
```bash
ffmpeg -fflags nobuffer -flags low_delay \
  -probesize 32 -analyzeduration 0 \
  -i rtp://239.0.0.1:5004 \
  -c copy \
  -f mp4 recording.mp4
```

## 3. TESTING & DEBUGGING

### 3.1 Test Stream Info
```bash
ffprobe -v error -show_format -show_streams rtp://239.0.0.1:5004
```

### 3.2 Monitor Bitrate
```bash
ffmpeg -i rtp://239.0.0.1:5004 -f null - 2>&1 | grep bitrate
```

### 3.3 Check Multicast Route (Windows)
```powershell
route print
netstat -g
```

### 3.4 Test Multicast Connectivity
```bash
# Sender (Teacher)
ffmpeg -f lavfi -i testsrc=size=1280x720:rate=30 \
  -c:v libx264 -preset ultrafast -tune zerolatency \
  -b:v 2M -f rtp_mpegts rtp://239.0.0.1:5004?ttl=1

# Receiver (Student)
ffplay rtp://239.0.0.1:5004
```

## 4. PRODUCTION DEPLOYMENT

### 4.1 Teacher Batch Script (teacher_start.bat)
```batch
@echo off
echo Starting Screen Share Server...
echo Streaming to: 239.0.0.1:5004
echo Press Ctrl+C to stop

ffmpeg -f gdigrab -framerate 30 -i desktop ^
  -c:v libx264 -preset ultrafast -tune zerolatency ^
  -profile:v baseline -level 3.1 -pix_fmt yuv420p ^
  -g 60 -keyint_min 60 ^
  -b:v 4M -maxrate 5M -bufsize 2M ^
  -f rtp_mpegts rtp://239.0.0.1:5004?ttl=1

pause
```

### 4.2 Student Batch Script (student_start.bat)
```batch
@echo off
echo Connecting to Teacher Screen...
echo Multicast: 239.0.0.1:5004
echo Press Q to quit

ffplay -fflags nobuffer -flags low_delay -framedrop ^
  -probesize 32 -analyzeduration 0 ^
  -hwaccel auto ^
  -fs ^
  rtp://239.0.0.1:5004

pause
```

## 5. PARAMETER TUNING

### 5.1 Encoding Presets (Speed vs Quality)
- `ultrafast`: Fastest, lowest CPU, lower quality
- `veryfast`: Good balance for real-time
- `faster`: Better quality, higher CPU
- `fast`: High quality, not recommended for real-time

### 5.2 Bitrate Guidelines
| Resolution | Bitrate | Max Bitrate | Buffer |
|------------|---------|-------------|--------|
| 1280x720   | 2-3 Mbps | 4 Mbps | 1.5 MB |
| 1600x900   | 3-4 Mbps | 5 Mbps | 2 MB |
| 1920x1080  | 4-5 Mbps | 6 Mbps | 2.5 MB |
| 2560x1440  | 6-8 Mbps | 10 Mbps | 4 MB |

### 5.3 GOP (Keyframe Interval)
- 30 frames (1s @ 30fps): Low latency, higher bitrate
- 60 frames (2s @ 30fps): Balanced (recommended)
- 90 frames (3s @ 30fps): Lower bitrate, higher latency

### 5.4 Framerate
- 25 FPS: Lower bandwidth, acceptable for presentations
- 30 FPS: Standard, recommended
- 60 FPS: Smooth, but doubles bandwidth

## 6. TROUBLESHOOTING COMMANDS

### 6.1 Check FFmpeg GPU Support
```bash
# List available encoders
ffmpeg -encoders | grep h264

# List hardware acceleration methods
ffmpeg -hwaccels

# Test NVENC
ffmpeg -f lavfi -i testsrc -c:v h264_nvenc -f null -

# Test QuickSync
ffmpeg -f lavfi -i testsrc -c:v h264_qsv -f null -
```

### 6.2 Network Diagnostics
```powershell
# Windows - Check multicast membership
netsh interface ipv4 show joins

# Add firewall rule
netsh advfirewall firewall add rule name="Multicast Screen Share" ^
  dir=in action=allow protocol=UDP localport=5004

# Test UDP connectivity
# Sender
ffmpeg -re -f lavfi -i testsrc -f mpegts udp://239.0.0.1:5004?ttl=1
# Receiver
ffplay udp://239.0.0.1:5004
```

## 7. PERFORMANCE MONITORING

### 7.1 Real-time Stats
```bash
# Teacher - Monitor encoding performance
ffmpeg -f gdigrab -i desktop \
  -c:v libx264 -preset ultrafast \
  -f rtp_mpegts rtp://239.0.0.1:5004?ttl=1 \
  -progress pipe:1 2>&1 | grep -E "fps|bitrate|speed"

# Student - Monitor decoding performance
ffplay -i rtp://239.0.0.1:5004 \
  -vf "drawtext=text='FPS\: %{pts}':x=10:y=10"
```
