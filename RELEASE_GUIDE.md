# HƯỚNG DẪN RELEASE

## 🚀 TỰ ĐỘNG RELEASE VỚI GITHUB ACTIONS

### Workflow đã setup:

1. **release.yml** - Tự động build và release khi push tag
2. **build-test.yml** - Test build khi push code (không release)

## 📋 CÁCH TẠO RELEASE

### Bước 1: Commit code

```bash
git add .
git commit -m "feat: add new features"
git push origin main
```

### Bước 2: Tạo và push tag

```bash
# Tạo tag với version mới
git tag v1.0.0

# Hoặc tag với message
git tag -a v1.0.0 -m "Release version 1.0.0"

# Push tag lên GitHub
git push origin v1.0.0
```

### Bước 3: Chờ GitHub Actions build

- GitHub Actions tự động trigger
- Download FFmpeg
- Build Windows installer
- Tạo GitHub Release
- Upload installer files

### Bước 4: Download installer

Vào GitHub Releases page:
```
https://github.com/YOUR_USERNAME/YOUR_REPO/releases
```

Download file:
- `screensharing-udp-ffmpeg_1.0.0_x64-setup.exe` (NSIS installer)
- `screensharing-udp-ffmpeg_1.0.0_x64_en-US.msi` (MSI installer)

## 🏷️ VERSION NAMING

Sử dụng Semantic Versioning:

```
v1.0.0 - Major release
v1.1.0 - Minor release (new features)
v1.1.1 - Patch release (bug fixes)
```

### Ví dụ:

```bash
# First release
git tag v1.0.0
git push origin v1.0.0

# Bug fix
git tag v1.0.1
git push origin v1.0.1

# New feature
git tag v1.1.0
git push origin v1.1.0

# Breaking changes
git tag v2.0.0
git push origin v2.0.0
```

## 📦 BUILD ARTIFACTS

Sau khi build thành công, GitHub Actions sẽ tạo:

### 1. GitHub Release
- Tự động tạo release page
- Generate release notes từ commits
- Attach installer files

### 2. Artifacts (lưu 30 ngày)
- NSIS installer (.exe)
- MSI installer (.msi)
- Có thể download từ Actions tab

## 🔍 KIỂM TRA BUILD STATUS

### Xem workflow đang chạy:
```
https://github.com/YOUR_USERNAME/YOUR_REPO/actions
```

### Check logs:
1. Click vào workflow run
2. Click vào job "Build and Release Windows"
3. Xem từng step

## ⚙️ WORKFLOW DETAILS

### release.yml (Trigger: Push tag)

**Steps:**
1. ✓ Checkout code
2. ✓ Setup Node.js 20
3. ✓ Setup Rust toolchain (stable)
4. ✓ Cache Rust dependencies
5. ✓ Install npm dependencies
6. ✓ Download FFmpeg (~150 MB)
7. ✓ Build Tauri app
8. ✓ Create GitHub Release
9. ✓ Upload installers

**Time:** ~15-20 phút

### build-test.yml (Trigger: Push code)

**Steps:**
1. ✓ Checkout code
2. ✓ Setup Node.js & Rust
3. ✓ Download FFmpeg
4. ✓ Build app (test only)
5. ✓ Verify artifacts

**Time:** ~15-20 phút

## 🐛 TROUBLESHOOTING

### Build failed - FFmpeg download error

**Nguyên nhân:** Network timeout hoặc URL thay đổi

**Giải pháp:**
- Retry workflow
- Hoặc update FFmpeg URL trong workflow

### Build failed - Rust compilation error

**Nguyên nhân:** Code lỗi hoặc dependencies issue

**Giải pháp:**
- Fix code locally
- Test build: `npm run tauri build`
- Commit và push lại

### Release không tạo được

**Nguyên nhân:** Permissions issue

**Giải pháp:**
- Check repository settings
- Ensure Actions có quyền write
- Settings → Actions → General → Workflow permissions → Read and write

### Tag đã tồn tại

**Nguyên nhân:** Tag đã được push trước đó

**Giải pháp:**
```bash
# Xóa tag local
git tag -d v1.0.0

# Xóa tag remote
git push origin :refs/tags/v1.0.0

# Tạo lại tag mới
git tag v1.0.0
git push origin v1.0.0
```

## 📝 CHECKLIST TRƯỚC KHI RELEASE

- [ ] Code đã test kỹ locally
- [ ] Version number đã update trong `src-tauri/tauri.conf.json`
- [ ] CHANGELOG.md đã update (nếu có)
- [ ] Commit message rõ ràng
- [ ] Tag version đúng format (vX.Y.Z)

## 🎯 BEST PRACTICES

### 1. Test trước khi release
```bash
# Build local để test
npm run tauri build

# Test installer
# Cài và chạy thử trên Windows
```

### 2. Update version trong tauri.conf.json
```json
{
  "version": "1.0.0"
}
```

### 3. Viết release notes tốt
```bash
git tag -a v1.0.0 -m "Release v1.0.0

Features:
- Add Teacher mode with DXGI capture
- Add Student mode with FFplay
- Bundle FFmpeg in installer

Bug Fixes:
- Fix streaming lag issue
- Fix firewall detection

Breaking Changes:
- None
"
```

### 4. Semantic versioning
- **Major (v2.0.0)**: Breaking changes
- **Minor (v1.1.0)**: New features, backward compatible
- **Patch (v1.0.1)**: Bug fixes only

## 🚀 DEPLOYMENT WORKFLOW

```
1. Development
   ├─ Code changes
   ├─ Test locally
   └─ Push to main
        └─ build-test.yml runs (verify build)

2. Release
   ├─ Create tag (v1.0.0)
   ├─ Push tag
   └─ release.yml runs
        ├─ Build Windows installer
        ├─ Create GitHub Release
        └─ Upload installers

3. Distribution
   ├─ Download installer from GitHub Releases
   ├─ Copy to USB
   └─ Deploy to 50 machines
```

## 📊 EXAMPLE RELEASE TIMELINE

```
09:00 - Commit final changes
09:05 - Push to main
09:10 - build-test.yml completes ✓
09:15 - Create and push tag v1.0.0
09:20 - release.yml starts
09:35 - Build completes
09:36 - Release created ✓
09:40 - Download installer
10:00 - Deploy to classroom
```

## 🔗 USEFUL LINKS

- GitHub Actions Docs: https://docs.github.com/en/actions
- Tauri Build Docs: https://tauri.app/v1/guides/building/
- Semantic Versioning: https://semver.org/

---

**Lưu ý:** Workflow chỉ build cho Windows. Không build macOS/Linux để tiết kiệm thời gian và resources.
