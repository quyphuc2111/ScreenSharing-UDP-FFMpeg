import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [mode, setMode] = useState<"select" | "teacher" | "student">("select");
  const [isStreaming, setIsStreaming] = useState(false);
  const [multicastAddr, setMulticastAddr] = useState("239.0.0.1:5004");
  const [bitrate, setBitrate] = useState("4M");
  const [status, setStatus] = useState("");
  const [ffmpegVersion, setFfmpegVersion] = useState("");

  useEffect(() => {
    checkFFmpeg();
  }, []);

  const checkFFmpeg = async () => {
    try {
      const version = await invoke<string>("check_ffmpeg");
      setFfmpegVersion(version);
      setStatus("✓ FFmpeg detected");
    } catch (error) {
      setFfmpegVersion("Not found");
      setStatus("⚠ FFmpeg not found. Please install FFmpeg and add to PATH.");
    }
  };

  const startTeacherStream = async () => {
    try {
      setStatus("Starting streaming...");
      const result = await invoke<string>("start_teacher_stream", {
        multicastAddr,
        bitrate,
      });
      setIsStreaming(true);
      setStatus(`✓ ${result}`);
    } catch (error) {
      setStatus(`✗ Error: ${error}`);
    }
  };

  const stopTeacherStream = async () => {
    try {
      setStatus("Stopping streaming...");
      const result = await invoke<string>("stop_teacher_stream");
      setIsStreaming(false);
      setStatus(`✓ ${result}`);
    } catch (error) {
      setStatus(`✗ Error: ${error}`);
    }
  };

  const startStudentView = async () => {
    try {
      setStatus("Starting student view...");
      const result = await invoke<string>("start_student_view", {
        multicastAddr,
      });
      setStatus(`✓ ${result}`);
    } catch (error) {
      setStatus(`✗ Error: ${error}`);
    }
  };

  if (mode === "select") {
    return (
      <div className="container">
        <h1>Screen Sharing System</h1>
        <div className="mode-selection">
          <div className="card">
            <h2>👨‍🏫 Teacher Mode</h2>
            <p>Share your screen to students</p>
            <button onClick={() => setMode("teacher")}>Select Teacher</button>
          </div>
          <div className="card">
            <h2>👨‍🎓 Student Mode</h2>
            <p>View teacher's screen</p>
            <button onClick={() => setMode("student")}>Select Student</button>
          </div>
        </div>
        <div className="status">
          <p>FFmpeg: {ffmpegVersion}</p>
          <p>{status}</p>
        </div>
      </div>
    );
  }

  if (mode === "teacher") {
    return (
      <div className="container">
        <button className="back-btn" onClick={() => setMode("select")}>
          ← Back
        </button>
        <h1>👨‍🏫 Teacher Mode</h1>
        
        <div className="config-section">
          <h3>Configuration</h3>
          <div className="form-group">
            <label>Multicast Address:</label>
            <input
              type="text"
              value={multicastAddr}
              onChange={(e) => setMulticastAddr(e.target.value)}
              disabled={isStreaming}
              placeholder="239.0.0.1:5004"
            />
          </div>
          <div className="form-group">
            <label>Bitrate:</label>
            <select
              value={bitrate}
              onChange={(e) => setBitrate(e.target.value)}
              disabled={isStreaming}
            >
              <option value="2M">2 Mbps (Low)</option>
              <option value="3M">3 Mbps (Medium)</option>
              <option value="4M">4 Mbps (High)</option>
              <option value="5M">5 Mbps (Very High)</option>
            </select>
          </div>
        </div>

        <div className="control-section">
          {!isStreaming ? (
            <button className="btn-primary" onClick={startTeacherStream}>
              🎥 Start Streaming
            </button>
          ) : (
            <button className="btn-danger" onClick={stopTeacherStream}>
              ⏹ Stop Streaming
            </button>
          )}
        </div>

        <div className="status-box">
          <h3>Status</h3>
          <div className={`status-indicator ${isStreaming ? "active" : ""}`}>
            {isStreaming ? "🔴 LIVE" : "⚫ Offline"}
          </div>
          <p>{status}</p>
        </div>

        <div className="info-box">
          <h3>ℹ Information</h3>
          <ul>
            <li>Screen will be captured at 30 FPS</li>
            <li>Students will connect to: {multicastAddr}</li>
            <li>Bandwidth usage: ~{bitrate.replace("M", "")} Mbps</li>
            <li>Latency: ~100-150ms</li>
          </ul>
        </div>
      </div>
    );
  }

  if (mode === "student") {
    return (
      <div className="container">
        <button className="back-btn" onClick={() => setMode("select")}>
          ← Back
        </button>
        <h1>👨‍🎓 Student Mode</h1>
        
        <div className="config-section">
          <h3>Configuration</h3>
          <div className="form-group">
            <label>Multicast Address:</label>
            <input
              type="text"
              value={multicastAddr}
              onChange={(e) => setMulticastAddr(e.target.value)}
              placeholder="239.0.0.1:5004"
            />
          </div>
        </div>

        <div className="control-section">
          <button className="btn-primary" onClick={startStudentView}>
            📺 Connect to Teacher
          </button>
        </div>

        <div className="status-box">
          <h3>Status</h3>
          <p>{status}</p>
        </div>

        <div className="info-box">
          <h3>ℹ Information</h3>
          <ul>
            <li>Connecting to: {multicastAddr}</li>
            <li>Video will open in FFplay window</li>
            <li>Press 'Q' in video window to quit</li>
            <li>Hardware acceleration: Auto-detected</li>
          </ul>
        </div>
      </div>
    );
  }

  return null;
}

export default App;
