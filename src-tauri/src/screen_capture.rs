// Screen Capture Module - DXGI Desktop Duplication
#[cfg(windows)]
use windows::{
    core::*,
    Win32::Graphics::{
        Direct3D::*,
        Direct3D11::*,
        Dxgi::{Common::*, *},
    },
    Win32::Foundation::*,
};

#[cfg(windows)]
pub struct ScreenCapturer {
    device: ID3D11Device,
    context: ID3D11DeviceContext,
    duplication: IDXGIOutputDuplication,
    staging_texture: ID3D11Texture2D,
    pub width: u32,
    pub height: u32,
}

#[cfg(windows)]
impl ScreenCapturer {
    pub fn new() -> Result<Self> {
        unsafe {
            // 1. Create D3D11 Device
            let mut device = None;
            let mut context = None;
            
            D3D11CreateDevice(
                None,
                D3D_DRIVER_TYPE_HARDWARE,
                None,
                D3D11_CREATE_DEVICE_FLAG(0),
                None,
                D3D11_SDK_VERSION,
                Some(&mut device),
                None,
                Some(&mut context),
            )?;
            
            let device: ID3D11Device = device.unwrap();
            let context = context.unwrap();

            // 2. Get DXGI Output (Monitor)
            let dxgi_device: IDXGIDevice = device.cast()?;
            let adapter = dxgi_device.GetAdapter()?;
            let output = adapter.EnumOutputs(0)?;
            let output1: IDXGIOutput1 = output.cast()?;

            // 3. Create Desktop Duplication
            let duplication = output1.DuplicateOutput(&device)?;

            // 4. Get screen dimensions from duplication desc
            let mut dupl_desc = DXGI_OUTDUPL_DESC::default();
            duplication.GetDesc(&mut dupl_desc);
            let width = dupl_desc.ModeDesc.Width;
            let height = dupl_desc.ModeDesc.Height;

            // 5. Create staging texture
            let staging_desc = D3D11_TEXTURE2D_DESC {
                Width: width,
                Height: height,
                MipLevels: 1,
                ArraySize: 1,
                Format: DXGI_FORMAT_B8G8R8A8_UNORM,
                SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
                Usage: D3D11_USAGE_STAGING,
                BindFlags: 0,
                CPUAccessFlags: D3D11_CPU_ACCESS_READ.0 as u32,
                MiscFlags: 0,
            };

            let mut staging_texture = None;
            device.CreateTexture2D(&staging_desc, None, Some(&mut staging_texture))?;

            Ok(Self {
                device,
                context,
                duplication,
                staging_texture: staging_texture.unwrap(),
                width,
                height,
            })
        }
    }

    pub fn capture_frame(&mut self) -> Result<Vec<u8>> {
        unsafe {
            // 1. Acquire next frame
            let mut frame_info = DXGI_OUTDUPL_FRAME_INFO::default();
            let mut desktop_resource = None;
            
            self.duplication.AcquireNextFrame(
                16, // 16ms timeout
                &mut frame_info,
                &mut desktop_resource,
            )?;

            // 2. Check if frame is new
            if frame_info.LastPresentTime == 0 {
                self.duplication.ReleaseFrame()?;
                return Err(Error::from(E_FAIL));
            }

            // 3. Get texture
            let desktop_texture: ID3D11Texture2D = desktop_resource.unwrap().cast()?;

            // 4. Copy to staging
            self.context.CopyResource(&self.staging_texture, &desktop_texture);

            // 5. Map and read data
            let mut mapped = D3D11_MAPPED_SUBRESOURCE::default();
            self.context.Map(&self.staging_texture, 0, D3D11_MAP_READ, 0, Some(&mut mapped))?;

            let row_pitch = mapped.RowPitch as usize;
            let data_size = (self.height as usize) * row_pitch;
            let mut frame_data = vec![0u8; data_size];

            std::ptr::copy_nonoverlapping(
                mapped.pData as *const u8,
                frame_data.as_mut_ptr(),
                data_size,
            );

            // 6. Cleanup
            self.context.Unmap(&self.staging_texture, 0);
            self.duplication.ReleaseFrame()?;

            Ok(frame_data)
        }
    }
}

#[cfg(not(windows))]
pub struct ScreenCapturer {
    pub width: u32,
    pub height: u32,
}

#[cfg(not(windows))]
impl ScreenCapturer {
    pub fn new() -> Result<Self, String> {
        Err("Screen capture only supported on Windows".to_string())
    }

    pub fn capture_frame(&mut self) -> Result<Vec<u8>, String> {
        Err("Screen capture only supported on Windows".to_string())
    }
}
