use std::path::PathBuf;

use coverwin_core::Format;
use serde::Serialize;
use tauri::ipc::Channel;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "status")]
enum Progress {
    Done {
        input: PathBuf,
        output: PathBuf,
        size: u64,
        input_size: u64,
    },
    Failed {
        input: PathBuf,
        error: String,
    },
}

/// Converte em sequência, reportando cada arquivo pelo `channel` assim que termina.
#[tauri::command]
async fn convert(
    files: Vec<PathBuf>,
    format: String,
    quality: u8,
    out_dir: Option<PathBuf>,
    channel: Channel<Progress>,
) -> Result<(), String> {
    let format: Format = format.parse().map_err(|e| format!("{e}"))?;
    tauri::async_runtime::spawn_blocking(move || {
        for input in files {
            let msg = match coverwin_core::convert(&input, format, quality, out_dir.as_deref()) {
                Ok(output) => Progress::Done {
                    size: std::fs::metadata(&output).map_or(0, |m| m.len()),
                    input_size: std::fs::metadata(&input).map_or(0, |m| m.len()),
                    input,
                    output,
                },
                Err(e) => Progress::Failed {
                    input,
                    error: e.to_string(),
                },
            };
            let _ = channel.send(msg);
        }
    })
    .await
    .map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![convert])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
