// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![convert_file,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn convert_file(path: String, resolution: Resolution) -> Result<String, String> {
    use std::path::Path;
    use std::process::Command;

    if !Path::new(&path).exists() {
        return Err("File does not exist".to_string());
    }

    let mut binding = Command::new("ffmpeg");
    let cmd = binding.arg("-i").arg(&path);

    let cmd = match resolution {
        Resolution::Sd => cmd.arg("-vf").arg("scale=-2:480"),
        Resolution::Hsd => cmd.arg("-vf").arg("scale=-2:600"),
        Resolution::Hd => cmd.arg("-vf").arg("scale=-2:720"),
        Resolution::Hdd => cmd.arg("-vf").arg("scale=-2:900"),
        Resolution::Same => cmd,
    };

    let path = Path::new(&path);

    let output_name = path.file_stem().unwrap().to_str().unwrap().to_string() + ".mp4";
    let output_path = path.parent().unwrap().join(output_name);

    cmd.arg(output_path.to_str().unwrap())
        .output()
        .expect("failed to execute process");

    Ok(output_path.to_str().unwrap().to_string())
}

#[derive(serde::Deserialize)]
enum Resolution {
    Sd,
    Hsd,
    Hd,
    Hdd,
    Same,
}
