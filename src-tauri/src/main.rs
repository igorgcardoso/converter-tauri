// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::Result as Res;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::time::Instant;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![convert_file,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn convert_file(path: String, resolution: Resolution, app: tauri::AppHandle) -> Result<(), String> {
    use std::path::Path;

    if !Path::new(&path).try_exists().unwrap_or_else(|_| false) {
        return Err("File does not exist".into());
    }

    let mut binding = Command::new("ffmpeg");
    let mut cmd = binding.args(&["-i", &path, "-f", "mp4", "-crf", "25", "-preset", "slow"]);
    let size = match resolution {
        Resolution::Sd => 480,
        Resolution::Hsd => 600,
        Resolution::Hd => 720,
        Resolution::Hdd => 900,
        Resolution::Same => return Ok(()),
    };

    cmd = match resolution {
        Resolution::Sd => cmd.arg("-vf").arg("scale=-2:480"),
        Resolution::Hsd => cmd.arg("-vf").arg("scale=-2:600"),
        Resolution::Hd => cmd.arg("-vf").arg("scale=-2:720"),
        Resolution::Hdd => cmd.arg("-vf").arg("scale=-2:900"),
        Resolution::Same => cmd,
    };

    let path = Path::new(&path);

    let output_name =
        path.file_stem().unwrap().to_str().unwrap().to_string() + &*format!("{size}p.mp4");
    let output_path = path.parent().unwrap().join(output_name);

    cmd = cmd.arg(output_path.to_str().unwrap());

    let _ = convert_file_and_process_output(cmd, app);

    Ok(())
}

#[tokio::main]
async fn convert_file_and_process_output(cmd: &mut Command, app: tauri::AppHandle) -> Res<()> {
    use tauri::Manager;

    let mut child = cmd.spawn()?;

    let stdout = match child.stdout.take() {
        Some(stdout) => stdout,
        None => return Err(anyhow::anyhow!("Failed to get stdout")),
    };
    let reader = BufReader::new(stdout);
    let mut lines = reader.lines();
    let start_time = Instant::now();

    while let Some(line) = lines.next_line().await? {
        if line.contains("frame=") {
            let current_frame: i64 = line.split("=").nth(1).unwrap().trim().parse()?;
            let total_frames: i64 = line.split(" ").nth(3).unwrap().trim().parse()?;
            let elapsed_time = start_time.elapsed().as_secs_f64();
            let eta = (elapsed_time / current_frame as f64) * (total_frames - current_frame) as f64;

            app.emit_all("eta-updated", eta)?;
        }
    }

    Ok(())
}

#[derive(serde::Deserialize)]
enum Resolution {
    Sd,
    Hsd,
    Hd,
    Hdd,
    Same,
}
