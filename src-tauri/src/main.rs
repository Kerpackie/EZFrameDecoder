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

#[tauri::command]
fn append_command(new_cmd: serde_json::Value) -> Result<(), String> {
    use serde_json::Value;

    // Load override file
    let path = ensure_user_spec().map_err(|e| e.to_string())?;
    let mut spec: Value =
        serde_json::from_str(&std::fs::read_to_string(&path).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;

    let commands = spec
        .get_mut("commands")
        .and_then(|v| v.as_array_mut())
        .ok_or("Malformed spec: 'commands' not array")?;

    /* -------------------------------------------------- */
    /* Case 1 – new top-level command                     */
    /* -------------------------------------------------- */
    if new_cmd.get("items").is_some() {
        let letter = new_cmd
            .get("letter")
            .and_then(|v| v.as_str())
            .ok_or("missing letter")?;

        if commands
            .iter()
            .any(|c| c.get("letter").and_then(|v| v.as_str()) == Some(letter))
        {
            return Err(format!(
                "Command with letter '{}' already exists. Use sub-command mode.",
                letter
            ));
        }

        commands.push(new_cmd);
    }
    /* -------------------------------------------------- */
    /* Case 2 – merge into existing switch                */
    /* new_cmd must have { letter, switch_key, switch_case, case } */
    /* -------------------------------------------------- */
    else {
        let letter       = new_cmd["letter"].as_str().ok_or("missing letter")?;
        let switch_key   = new_cmd["switch_key"]
            .as_str()
            .unwrap_or("Opcode");
        let switch_case  = new_cmd["switch_case"]
            .as_str()
            .ok_or("missing switch_case")?;
        let case_obj     = new_cmd["case"].clone();

        let cmd = commands
            .iter_mut()
            .find(|c| c["letter"] == letter)
            .ok_or("Base command not found")?;

        let switch_item = cmd["items"]
            .as_array_mut()
            .ok_or("items not array")?
            .iter_mut()
            .find(|it| it.get("switch").and_then(|v| v.as_str()) == Some(switch_key))
            .ok_or("No switch with that key")?;

        let cases = switch_item["cases"]
            .as_object_mut()
            .ok_or("cases not object")?;

        if cases.contains_key(switch_case) {
            return Err("That switch_case already exists".into());
        }

        cases.insert(switch_case.to_string(), case_obj);
    }

    std::fs::write(&path, serde_json::to_string_pretty(&spec).unwrap())
        .map_err(|e| e.to_string())?;

    // Hot-reload
    reload_spec()?;
    Ok(())
}

/* ─────────────── main entry ─────────────── */

fn main() {
    tauri::Builder::default()
        .plugin(fs_plugin()) // keeps file-dialog APIs available
        .invoke_handler(tauri::generate_handler![
            decode_frame,
            batch_decode,
            reload_spec,
            append_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running Tauri app");
}
