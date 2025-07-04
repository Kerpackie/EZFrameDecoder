#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod decoder;

use decoder::{decode, SpecFile};
use once_cell::sync::Lazy;
use std::{fs, path::PathBuf, sync::RwLock};
use tauri_plugin_fs::init as fs_plugin;
use dirs_next::config_dir;

/* ───────────── constants ───────────── */
const APP_DIR: &str = "EZFrameDecoder";
const USER_SPEC_FILE: &str = "spec_override.json";

/* Bundled default, embedded at compile-time */
const DEFAULT_SPEC: &str = include_str!("../resources/spec_full.json");

/* ───── copy default to user folder if missing ───── */
fn ensure_user_spec() -> std::io::Result<PathBuf> {
    let user_path = config_dir()
        .ok_or(std::io::ErrorKind::NotFound)?
        .join(APP_DIR)
        .join(USER_SPEC_FILE);

    if !user_path.exists() {
        if let Some(parent) = user_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&user_path, DEFAULT_SPEC)?;
        println!("★ Copied default spec to {}", user_path.display());
    }

    Ok(user_path)
}

/* ───── load (or reload) spec from disk ───── */
fn load_spec_from_disk() -> Result<SpecFile, Box<dyn std::error::Error>> {
    let path = ensure_user_spec()?;
    let text = fs::read_to_string(&path)?;
    let spec = serde_json::from_str::<SpecFile>(&text)?;
    Ok(spec)
}

/* ───── shared, hot-swappable spec instance ───── */
static SPEC: Lazy<RwLock<SpecFile>> =
    Lazy::new(|| RwLock::new(load_spec_from_disk().expect("load spec")));

/* ───────────── Tauri commands ───────────── */

fn clean_frame(line: &str) -> &str {
    line.split_whitespace().next().unwrap_or("")
}

#[tauri::command]
fn decode_frame(frame: String) -> Result<serde_json::Value, String> {
    let cleaned = clean_frame(&frame);
    if !cleaned.starts_with('<') {
        return Err("Line does not start with '<'".into());
    }
    let spec = SPEC.read().unwrap();
    decode(cleaned, &*spec).map_err(|e| e.to_string())
}

#[tauri::command]
fn batch_decode(text: String) -> Result<Vec<serde_json::Value>, String> {
    let spec = SPEC.read().unwrap();
    text.lines()
        .filter_map(|raw| {
            let cleaned = clean_frame(raw.trim());
            if cleaned.starts_with('<') && !cleaned.is_empty() {
                Some(
                    decode(cleaned, &*spec)
                        .map_err(|e| e.to_string())
                )
            } else {
                None // skip non-frame lines
            }
        })
        .collect()
}

#[tauri::command]
fn reload_spec() -> Result<(), String> {
    let mut spec_lock = SPEC.write().unwrap();
    *spec_lock = load_spec_from_disk().map_err(|e| e.to_string())?;
    Ok(())
}

/* ─────────────── main entry ─────────────── */

fn main() {
    tauri::Builder::default()
        .plugin(fs_plugin()) // keeps file-dialog APIs available
        .invoke_handler(tauri::generate_handler![
            decode_frame,
            batch_decode,
            reload_spec
        ])
        .run(tauri::generate_context!())
        .expect("error while running Tauri app");
}
