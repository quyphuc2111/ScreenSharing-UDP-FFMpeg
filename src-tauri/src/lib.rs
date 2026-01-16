mod screen_capture;
mod streaming;

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use tauri::State;

struct AppState {
    streaming_server: Arc<Mutex<Option<streaming::StreamingServer>>>,
    is_streaming: Arc<Mutex<bool>>,
}

#[derive(serde::Serialize, Clone)]
struct StreamStats {
    frames: u64,
    fps: f64,
    is_running: bool,
}

#[tauri::command]
async fn start_teacher_stream(
    state: State<'_, AppState>,
    multicast_addr: String,
    bitrate: String,
) -> Result<String, String> {
    let mut is_streaming = state.is_streaming.lock().unwrap();
    if *is_streaming {
        return Err("Already streaming".to_string());
    }

    #[cfg(windows)]
    {
        // Initialize screen capturer
        let mut capturer = screen_capture::ScreenCapturer::new()
            .map_err(|e| format!("Failed to initialize screen capture: {:?}", e))?;

        let width = capturer.width;
        let height = capturer.height;
        let fps = 30u32;

        // Initialize streaming server
        let mut server = streaming::StreamingServer::new();
        server.start(width, height, fps, &multicast_addr, &bitrate)
            .map_err(|e| format!("Failed to start streaming: {}", e))?;

        *is_streaming = true;
        let is_streaming_clone = state.is_streaming.clone();

        // Spawn capture thread
        thread::spawn(move || {
            let frame_duration = Duration::from_millis(1000 / fps as u64);
            let mut frame_count = 0u64;
            let start_time = Instant::now();

            while *is_streaming_clone.lock().unwrap() {
                let loop_start = Instant::now();

                match capturer.capture_frame() {
                    Ok(frame_data) => {
                        if let Err(e) = server.send_frame(&frame_data) {
                            eprintln!("Failed to send frame: {}", e);
                            break;
                        }
                        frame_count += 1;

                        if frame_count % (fps as u64 * 5) == 0 {
                            let elapsed = start_time.elapsed().as_secs_f64();
                            let actual_fps = frame_count as f64 / elapsed;
                            println!("Frames: {} | FPS: {:.1}", frame_count, actual_fps);
                        }
                    }
                    Err(_) => {
                        thread::sleep(Duration::from_millis(1));
                        continue;
                    }
                }

                let elapsed = loop_start.elapsed();
                if elapsed < frame_duration {
                    thread::sleep(frame_duration - elapsed);
                }
            }

            let _ = server.stop();
            println!("Streaming stopped");
        });

        Ok(format!("Streaming started: {}x{} @ {} fps to {}", width, height, fps, multicast_addr))
    }

    #[cfg(not(windows))]
    {
        Err("Screen capture only supported on Windows".to_string())
    }
}

#[tauri::command]
async fn stop_teacher_stream(state: State<'_, AppState>) -> Result<String, String> {
    let mut is_streaming = state.is_streaming.lock().unwrap();
    if !*is_streaming {
        return Err("Not streaming".to_string());
    }

    *is_streaming = false;
    Ok("Streaming stopped".to_string())
}

#[tauri::command]
async fn get_stream_status(state: State<'_, AppState>) -> Result<bool, String> {
    Ok(*state.is_streaming.lock().unwrap())
}

#[tauri::command]
async fn start_student_view(multicast_addr: String) -> Result<String, String> {
    // Get FFplay path (bundled or system)
    let ffplay_path = streaming::get_ffplay_path()
        .map_err(|e| e.to_string())?;

    let ffplay_path_display = ffplay_path.display().to_string();

    // Spawn FFplay in a new process
    thread::spawn(move || {
        let _ = std::process::Command::new(&ffplay_path)
            .args(&[
                "-fflags", "nobuffer",
                "-flags", "low_delay",
                "-framedrop",
                "-probesize", "32",
                "-analyzeduration", "0",
                "-hwaccel", "auto",
                "-window_title", "Teacher Screen",
                &format!("rtp://{}", multicast_addr),
            ])
            .spawn();
    });

    Ok(format!("Student view started (using {})", ffplay_path_display))
}

#[tauri::command]
fn check_ffmpeg() -> Result<String, String> {
    streaming::check_ffmpeg().map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            streaming_server: Arc::new(Mutex::new(None)),
            is_streaming: Arc::new(Mutex::new(false)),
        })
        .invoke_handler(tauri::generate_handler![
            start_teacher_stream,
            stop_teacher_stream,
            get_stream_status,
            start_student_view,
            check_ffmpeg,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
