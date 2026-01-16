# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial release
- Teacher mode with DXGI screen capture
- Student mode with FFplay viewer
- UDP Multicast streaming (H.264)
- FFmpeg bundled in installer
- Modern UI with gradient design
- Support for 30-50 students
- Automatic firewall configuration guide

### Technical
- DXGI Desktop Duplication API for screen capture
- FFmpeg H.264 encoding (libx264/NVENC)
- RTP over UDP Multicast transport
- GPU hardware decoding (DXVA2)
- DirectX rendering

## [1.0.0] - YYYY-MM-DD

### Added
- First stable release
- Windows 10/11 support
- Teacher and Student modes
- Real-time screen sharing
- Low latency (~100-150ms)
- Fixed bandwidth (5 Mbps)
- Scalable to 50+ students

### Features
- DXGI screen capture (Windows native)
- H.264 video encoding
- UDP Multicast streaming
- GPU hardware acceleration
- Modern glassmorphism UI
- Configurable bitrate (2-5 Mbps)
- Automatic FFmpeg detection

### Documentation
- Quick start guide
- Deployment guide
- System design documentation
- Architecture diagrams
- Troubleshooting guide

---

## Release Notes Template

```markdown
## [X.Y.Z] - YYYY-MM-DD

### Added
- New features

### Changed
- Changes in existing functionality

### Deprecated
- Soon-to-be removed features

### Removed
- Removed features

### Fixed
- Bug fixes

### Security
- Security fixes
```
