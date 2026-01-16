# SƠ ĐỒ KIẾN TRÚC HỆ THỐNG

## 1. LUỒNG DỮ LIỆU TỔNG THỂ

```
┌─────────────────────────────────────────────────────────────────────┐
│                         TEACHER MACHINE                             │
│                                                                     │
│  ┌──────────────────────────────────────────────────────────────┐ │
│  │  STEP 1: SCREEN CAPTURE (DXGI Desktop Duplication)          │ │
│  │  ─────────────────────────────────────────────────────────   │ │
│  │  • Windows API: IDXGIOutputDuplication                       │ │
│  │  • Capture: 1920x1080 @ 30 FPS                              │ │
│  │  • Format: BGRA (4 bytes/pixel)                             │ │
│  │  • Output: Raw frames in GPU memory                         │ │
│  │  • CPU Usage: ~5%                                            │ │
│  └────────────────────┬─────────────────────────────────────────┘ │
│                       │                                             │
│                       ▼                                             │
│  ┌──────────────────────────────────────────────────────────────┐ │
│  │  STEP 2: VIDEO ENCODING (FFmpeg libx264)                    │ │
│  │  ─────────────────────────────────────────────────────────   │ │
│  │  • Codec: H.264                                              │ │
│  │  • Profile: Baseline                                         │ │
│  │  • Preset: ultrafast / veryfast                             │ │
│  │  • Bitrate: 4-5 Mbps (CBR/VBR)                              │ │
│  │  • GOP: 60 frames (2 seconds)                               │ │
│  │  • Pixel Format: YUV420p                                     │ │
│  │  • CPU Usage: ~15-20% (1 core)                              │ │
│  │  • Alternative: NVENC/QuickSync (GPU encoding)              │ │
│  └────────────────────┬─────────────────────────────────────────┘ │
│                       │                                             │
│                       ▼                                             │
│  ┌──────────────────────────────────────────────────────────────┐ │
│  │  STEP 3: RTP PACKETIZATION                                  │ │
│  │  ─────────────────────────────────────────────────────────   │ │
│  │  • Protocol: RTP (Real-time Transport Protocol)             │ │
│  │  • Container: MPEG-TS (Transport Stream)                    │ │
│  │  • Packet Size: ~1400 bytes (MTU-safe)                      │ │
│  │  • Sequence Numbers: For ordering                           │ │
│  │  • Timestamps: For synchronization                          │ │
│  └────────────────────┬─────────────────────────────────────────┘ │
│                       │                                             │
│                       ▼                                             │
│  ┌──────────────────────────────────────────────────────────────┐ │
│  │  STEP 4: UDP MULTICAST TRANSMISSION                         │ │
│  │  ─────────────────────────────────────────────────────────   │ │
│  │  • Destination: 239.0.0.1:5004                              │ │
│  │  • TTL: 1 (LAN only)                                        │ │
│  │  • Bandwidth: ~5 Mbps (constant)                            │ │
│  │  • No ACK, No Retry (fire-and-forget)                       │ │
│  └────────────────────┬─────────────────────────────────────────┘ │
│                       │                                             │
└───────────────────────┼─────────────────────────────────────────────┘
                        │
                        │ Single Stream (5 Mbps)
                        │
                        ▼
        ┌───────────────────────────────────┐
        │     LAN SWITCH (Gigabit)          │
        │  • IGMP Snooping: Enabled         │
        │  • Multicast Filtering: Enabled   │
        │  • Replicates packets to all      │
        │    members of multicast group     │
        └───────────────┬───────────────────┘
                        │
        ┌───────────────┴───────────────┬───────────────┬─────────────┐
        │                               │               │             │
        ▼                               ▼               ▼             ▼
┌───────────────┐              ┌───────────────┐   ┌───────────────┐ ...
│  STUDENT 1    │              │  STUDENT 2    │   │  STUDENT 50   │
│               │              │               │   │               │
│  ┌─────────┐  │              │  ┌─────────┐  │   │  ┌─────────┐  │
│  │ STEP 1: │  │              │  │ STEP 1: │  │   │  │ STEP 1: │  │
│  │ JOIN    │  │              │  │ JOIN    │  │   │  │ JOIN    │  │
│  │ MCAST   │  │              │  │ MCAST   │  │   │  │ MCAST   │  │
│  │ GROUP   │  │              │  │ GROUP   │  │   │  │ GROUP   │  │
│  └────┬────┘  │              │  └────┬────┘  │   │  └────┬────┘  │
│       │       │              │       │       │   │       │       │
│       ▼       │              │       ▼       │   │       ▼       │
│  ┌─────────┐  │              │  ┌─────────┐  │   │  ┌─────────┐  │
│  │ STEP 2: │  │              │  │ STEP 2: │  │   │  │ STEP 2: │  │
│  │ RECEIVE │  │              │  │ RECEIVE │  │   │  │ RECEIVE │  │
│  │ RTP     │  │              │  │ RTP     │  │   │  │ RTP     │  │
│  │ PACKETS │  │              │  │ PACKETS │  │   │  │ PACKETS │  │
│  └────┬────┘  │              │  └────┬────┘  │   │  └────┬────┘  │
│       │       │              │       │       │   │       │       │
│       ▼       │              │       ▼       │   │       ▼       │
│  ┌─────────┐  │              │  ┌─────────┐  │   │  ┌─────────┐  │
│  │ STEP 3: │  │              │  │ STEP 3: │  │   │  │ STEP 3: │  │
│  │ RTP     │  │              │  │ RTP     │  │   │  │ RTP     │  │
│  │ DEPACK  │  │              │  │ DEPACK  │  │   │  │ DEPACK  │  │
│  └────┬────┘  │              │  └────┬────┘  │   │  └────┬────┘  │
│       │       │              │       │       │   │       │       │
│       ▼       │              │       ▼       │   │       ▼       │
│  ┌─────────┐  │              │  ┌─────────┐  │   │  ┌─────────┐  │
│  │ STEP 4: │  │              │  │ STEP 4: │  │   │  │ STEP 4: │  │
│  │ GPU     │  │              │  │ GPU     │  │   │  │ GPU     │  │
│  │ DECODE  │  │              │  │ DECODE  │  │   │  │ DECODE  │  │
│  │ H.264   │  │              │  │ H.264   │  │   │  │ H.264   │  │
│  │ (DXVA2) │  │              │  │ (DXVA2) │  │   │  │ (DXVA2) │  │
│  └────┬────┘  │              │  └────┬────┘  │   │  └────┬────┘  │
│       │       │              │       │       │   │       │       │
│       ▼       │              │       ▼       │   │       ▼       │
│  ┌─────────┐  │              │  ┌─────────┐  │   │  ┌─────────┐  │
│  │ STEP 5: │  │              │  │ STEP 5: │  │   │  │ STEP 5: │  │
│  │ RENDER  │  │              │  │ RENDER  │  │   │  │ RENDER  │  │
│  │ DirectX │  │              │  │ DirectX │  │   │  │ DirectX │  │
│  │ Display │  │              │  │ Display │  │   │  │ Display │  │
│  └─────────┘  │              │  └─────────┘  │   │  └─────────┘  │
│               │              │               │   │               │
└───────────────┘              └───────────────┘   └───────────────┘
```

## 2. TIMING DIAGRAM (Độ trễ)

```
Time (ms)  Teacher                          Network              Student
─────────────────────────────────────────────────────────────────────────
0          │ Capture frame (DXGI)
           │ ├─ AcquireNextFrame()
16         │ └─ Frame ready (BGRA)
           │
           │ Encode H.264
           │ ├─ Color conversion
           │ ├─ Motion estimation
           │ ├─ Transform & quantization
56         │ └─ Encoded NAL units
           │
           │ RTP packetization
           │ ├─ Split into packets
61         │ └─ Add RTP headers
           │
           │ UDP send
           │ ├─ Socket write
66         │ └─ Packet on wire ──────────>  │
           │                                 │ Switch forwarding
           │                                 │ ├─ IGMP lookup
71         │                                 │ └─ Replicate ────>  │
           │                                                       │ Receive packets
           │                                                       │ ├─ Socket read
           │                                                       │ ├─ Buffer
76         │                                                       │ └─ RTP depack
           │                                                       │
           │                                                       │ GPU decode
           │                                                       │ ├─ H.264 decode
           │                                                       │ ├─ YUV → RGB
106        │                                                       │ └─ Frame ready
           │                                                       │
           │                                                       │ Render
           │                                                       │ ├─ Upload texture
           │                                                       │ ├─ Draw quad
116        │                                                       │ └─ Present
           │                                                       │
           │                                                       │ [DISPLAY]
           
TOTAL LATENCY: ~100-120ms (acceptable for classroom)
```

## 3. BANDWIDTH ANALYSIS

```
┌─────────────────────────────────────────────────────────────┐
│  TEACHER UPLOAD BANDWIDTH                                   │
│  ─────────────────────────────────────────────────────────  │
│                                                             │
│  Video bitrate:     4 Mbps                                  │
│  RTP overhead:      ~5% (200 Kbps)                          │
│  MPEG-TS overhead:  ~3% (120 Kbps)                          │
│  ─────────────────────────────────────────────────────────  │
│  TOTAL:            ~4.3-4.5 Mbps                            │
│                                                             │
│  ✓ CONSTANT regardless of student count                    │
│  ✓ 1 student = 4.5 Mbps                                    │
│  ✓ 50 students = 4.5 Mbps (same!)                          │
│                                                             │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│  SWITCH BANDWIDTH                                           │
│  ─────────────────────────────────────────────────────────  │
│                                                             │
│  Input from Teacher:  4.5 Mbps                              │
│  Output to Students:  4.5 Mbps × 50 = 225 Mbps             │
│                                                             │
│  ✓ Gigabit switch (1000 Mbps) handles easily               │
│  ✓ Even 100 Mbps switch can handle 20 students             │
│                                                             │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│  STUDENT DOWNLOAD BANDWIDTH                                 │
│  ─────────────────────────────────────────────────────────  │
│                                                             │
│  Per student:  4.5 Mbps                                     │
│                                                             │
│  ✓ Fast Ethernet (100 Mbps) sufficient                     │
│  ✓ Gigabit Ethernet (1000 Mbps) recommended                │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

## 4. CPU/GPU USAGE

```
TEACHER MACHINE:
┌────────────────────────────────────────────────────────┐
│  Component          │  Usage  │  Notes                 │
├────────────────────────────────────────────────────────┤
│  DXGI Capture       │  ~5%    │  Minimal overhead      │
│  H.264 Encode (CPU) │  15-25% │  1 core, ultrafast     │
│  H.264 Encode (GPU) │  ~10%   │  NVENC/QuickSync       │
│  RTP Packetization  │  ~2%    │  Negligible            │
│  Network Send       │  ~1%    │  Negligible            │
├────────────────────────────────────────────────────────┤
│  TOTAL (CPU encode) │  ~25%   │  i5-6500 or better     │
│  TOTAL (GPU encode) │  ~8%    │  Recommended           │
└────────────────────────────────────────────────────────┘

STUDENT MACHINE:
┌────────────────────────────────────────────────────────┐
│  Component          │  Usage  │  Notes                 │
├────────────────────────────────────────────────────────┤
│  Network Receive    │  ~1%    │  Negligible            │
│  RTP Depacketize    │  ~3%    │  Minimal               │
│  H.264 Decode (GPU) │  10-15% │  DXVA2/NVDEC           │
│  H.264 Decode (CPU) │  40-60% │  Fallback, not ideal   │
│  DirectX Render     │  ~5%    │  Minimal               │
├────────────────────────────────────────────────────────┤
│  TOTAL (GPU decode) │  ~15%   │  Recommended           │
│  TOTAL (CPU decode) │  ~50%   │  Avoid if possible     │
└────────────────────────────────────────────────────────┘
```

## 5. PACKET FLOW

```
TEACHER:
┌─────────────────────────────────────────────────────────┐
│  H.264 Encoded Frame (150 KB @ 4 Mbps, 30 fps)         │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
        ┌────────────────────────┐
        │  RTP Packetization     │
        │  ─────────────────────  │
        │  Split into ~110       │
        │  packets of 1400 bytes │
        └────────┬───────────────┘
                 │
                 ▼
    ┌────────────────────────────────┐
    │  RTP Packet Structure          │
    │  ────────────────────────────   │
    │  [12B RTP Header]              │
    │  [1388B H.264 Payload]         │
    │  ────────────────────────────   │
    │  Total: 1400 bytes             │
    └────────┬───────────────────────┘
             │
             ▼
┌────────────────────────────────────┐
│  UDP Packet                        │
│  ────────────────────────────────   │
│  [8B UDP Header]                   │
│  [1400B RTP Packet]                │
│  ────────────────────────────────   │
│  Total: 1408 bytes                 │
└────────┬───────────────────────────┘
         │
         ▼
┌────────────────────────────────────┐
│  IP Packet                         │
│  ────────────────────────────────   │
│  [20B IP Header]                   │
│  [1408B UDP Packet]                │
│  ────────────────────────────────   │
│  Total: 1428 bytes                 │
│  Destination: 239.0.0.1            │
└────────┬───────────────────────────┘
         │
         ▼
┌────────────────────────────────────┐
│  Ethernet Frame                    │
│  ────────────────────────────────   │
│  [14B Ethernet Header]             │
│  [1428B IP Packet]                 │
│  [4B CRC]                          │
│  ────────────────────────────────   │
│  Total: 1446 bytes                 │
│  < 1500 MTU ✓                      │
└────────────────────────────────────┘
```

## 6. ERROR HANDLING

```
PACKET LOSS SCENARIO:
─────────────────────────────────────────────────────────

Frame N:   [I-frame] ──────────────────> ✓ Received
           (Keyframe, complete)

Frame N+1: [P-frame] ──────────────────> ✓ Received
           (References Frame N)

Frame N+2: [P-frame] ──X───────────────> ✗ LOST
           (Some packets dropped)

Frame N+3: [P-frame] ──────────────────> ✓ Received
           (References Frame N+2)          ⚠ Cannot decode!
                                           (Missing reference)

Frame N+4: [P-frame] ──────────────────> ✓ Received
                                          ⚠ Cannot decode!

...

Frame N+60: [I-frame] ─────────────────> ✓ Received
            (New keyframe)                ✓ Resync!
                                          ✓ Decoding resumes

MITIGATION:
• Frequent keyframes (every 2 seconds)
• Error concealment in decoder
• Accept 1-2% packet loss
• No retry (UDP nature)
```

## 7. SCALABILITY

```
NUMBER OF STUDENTS vs PERFORMANCE:
───────────────────────────────────────────────────────

Teacher CPU:     ████████████████████ 25% (constant)
Teacher Network: ████████████████████ 5 Mbps (constant)

Students:  1    10   20   30   40   50
           │    │    │    │    │    │
Teacher:   ████████████████████████████ (no change)
Switch:    █    ██   ███  ████ █████████ (scales linearly)
Student:   ████████████████████████████ (constant per student)

✓ Teacher performance: O(1) - constant
✓ Switch bandwidth: O(n) - linear, but manageable
✓ Student performance: O(1) - constant per student

THEORETICAL LIMIT:
• Gigabit switch: 1000 Mbps / 5 Mbps = 200 students
• Practical limit: 50-100 students (network overhead)
```
