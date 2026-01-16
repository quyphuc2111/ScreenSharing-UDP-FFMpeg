// Streaming Module - FFmpeg Integration
use std::process::{Child, Command, Stdio};
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::path::PathBuf;
use anyhow::{Result, Context};

pub struct StreamingServer {
    ffmpeg_process: Option<Child>,
    stdin: Option<std::process::ChildStdin>,
    pub is_running: Arc<Mutex<bool>>,
}

impl StreamingServer {
    pub fn new() -> Self {
        Self {
            ffmpeg_process: None,
            stdin: None,
            is_running: Arc::new(Mutex::new(false)),
        }
    }

    fn get_ffmpeg_path() -> Result<PathBuf> {
        // Try bundled FFmpeg first (Windows)
        #[cfg(windows)]
        {
            if let Ok(exe_dir) = std::env::current_exe() {
                if let Some(parent) = exe_dir.parent() {
                    // Check in same directory as executable
                    let bundled_ffmpeg = parent.join("ffmpeg.exe");
                    if bundled_ffmpeg.exists() {
                        return Ok(bundled_ffmpeg);
                    }

                    // Check in resources/windows folder
                    let resources_ffmpeg = parent.join("resources").join("windows").join("ffmpeg.exe");
                    if resources_ffmpeg.exists() {
                        return Ok(resources_ffmpeg);
                    }
                }
            }

            // Fallback to system FFmpeg
            if Command::new("ffmpeg").arg("-version").output().is_ok() {
                return Ok(PathBuf::from("ffmpeg"));
            }

            return Err(anyhow::anyhow!(
                "FFmpeg not found. Please run: .\\download_ffmpeg.ps1"
            ));
        }

        // macOS/Linux - use system FFmpeg
        #[cfg(not(windows))]
        {
            if Command::new("ffmpeg").arg("-version").output().is_ok() {
                return Ok(PathBuf::from("ffmpeg"));
            }

            return Err(anyhow::anyhow!(
                "FFmpeg not found. Please install:\n\
                - macOS: brew install ffmpeg\n\
                - Linux: sudo apt install ffmpeg"
            ));
        }
    }

    pub fn start(
        &mut self,
        width: u32,
        height: u32,
        fps: u32,
        multicast_addr: &str,
        bitrate: &str,
    ) -> Result<()> {
        let ffmpeg_path = Self::get_ffmpeg_path()?;

        let mut ffmpeg = Command::new(&ffmpeg_path)
            // Input: raw BGRA frames from stdin
            .args(&[
                "-f", "rawvideo",
                "-pixel_format", "bgra",
                "-video_size", &format!("{}x{}", width, height),
                "-framerate", &fps.to_string(),
                "-i", "pipe:0",
            ])
            // Encoding settings
            .args(&[
                "-c:v", "libx264",
                "-preset", "ultrafast",
                "-tune", "zerolatency",
                "-profile:v", "baseline",
                "-level", "3.1",
                "-pix_fmt", "yuv420p",
            ])
            // GOP and keyframe
            .args(&[
                "-g", &(fps * 2).to_string(),
                "-keyint_min", &(fps * 2).to_string(),
            ])
            // Bitrate control
            .args(&[
                "-b:v", bitrate,
                "-maxrate", &format!("{}M", bitrate.trim_end_matches('M').parse::<u32>().unwrap_or(4) + 1),
                "-bufsize", "2M",
            ])
            // Output: RTP multicast
            .args(&[
                "-f", "rtp_mpegts",
                &format!("rtp://{}?ttl=1", multicast_addr),
            ])
            // Logging
            .args(&[
                "-loglevel", "warning",
            ])
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::inherit())
            .spawn()
            .context("Failed to start FFmpeg")?;

        let stdin = ffmpeg.stdin.take().unwrap();

        self.ffmpeg_process = Some(ffmpeg);
        self.stdin = Some(stdin);
        *self.is_running.lock().unwrap() = true;

        Ok(())
    }

    pub fn send_frame(&mut self, frame_data: &[u8]) -> Result<()> {
        if let Some(stdin) = &mut self.stdin {
            stdin.write_all(frame_data)?;
            stdin.flush()?;
            Ok(())
        } else {
            Err(anyhow::anyhow!("Streaming not started"))
        }
    }

    pub fn stop(&mut self) -> Result<()> {
        *self.is_running.lock().unwrap() = false;
        
        if let Some(stdin) = self.stdin.take() {
            drop(stdin);
        }

        if let Some(mut process) = self.ffmpeg_process.take() {
            process.kill()?;
            process.wait()?;
        }

        Ok(())
    }

    pub fn is_running(&self) -> bool {
        *self.is_running.lock().unwrap()
    }
}

impl Drop for StreamingServer {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}

// Helper function to check FFmpeg availability
pub fn check_ffmpeg() -> Result<String> {
    let ffmpeg_path = StreamingServer::get_ffmpeg_path()?;
    
    let output = Command::new(&ffmpeg_path)
        .arg("-version")
        .output()
        .context("Failed to execute FFmpeg")?;

    let version = String::from_utf8_lossy(&output.stdout);
    let first_line = version.lines().next().unwrap_or("Unknown version");
    
    Ok(format!("{} ({})", first_line, ffmpeg_path.display()))
}

// Helper function to get FFplay path
pub fn get_ffplay_path() -> Result<PathBuf> {
    // Try bundled FFplay first (Windows)
    #[cfg(windows)]
    {
        if let Ok(exe_dir) = std::env::current_exe() {
            if let Some(parent) = exe_dir.parent() {
                let bundled_ffplay = parent.join("ffplay.exe");
                if bundled_ffplay.exists() {
                    return Ok(bundled_ffplay);
                }

                let resources_ffplay = parent.join("resources").join("windows").join("ffplay.exe");
                if resources_ffplay.exists() {
                    return Ok(resources_ffplay);
                }
            }
        }

        // Fallback to system FFplay
        if Command::new("ffplay").arg("-version").output().is_ok() {
            return Ok(PathBuf::from("ffplay"));
        }

        return Err(anyhow::anyhow!(
            "FFplay not found. Please run: .\\download_ffmpeg.ps1"
        ));
    }

    // macOS/Linux - use system FFplay
    #[cfg(not(windows))]
    {
        if Command::new("ffplay").arg("-version").output().is_ok() {
            return Ok(PathBuf::from("ffplay"));
        }

        return Err(anyhow::anyhow!(
            "FFplay not found. Please install:\n\
            - macOS: brew install ffmpeg\n\
            - Linux: sudo apt install ffmpeg"
        ));
    }
}

