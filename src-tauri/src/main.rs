#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

mod decoder;

use decoder::{decode, SpecFile, Family};
use once_cell::sync::Lazy;
use std::{fs, path::PathBuf, sync::RwLock};
use tauri_plugin_fs::init as fs_plugin;
use dirs_next::config_dir;
use serde_json::json;

/* ───── constants ───── */
const APP_DIR: &str = "EZFrameDecoder";
const USER_SPEC_FILE: &str = "spec_override.json";
const DEFAULT_SPEC: &str = include_str!("../resources/spec_full.json");

/* ───── ensure spec file exists ───── */
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

/* ───── load spec file ───── */
fn load_spec_from_disk() -> Result<SpecFile, Box<dyn std::error::Error>> {
    let path = ensure_user_spec()?;
    let text = fs::read_to_string(&path)?;
    let spec = serde_json::from_str::<SpecFile>(&text)?;
    Ok(spec)
}

static SPEC: Lazy<RwLock<SpecFile>> =
    Lazy::new(|| RwLock::new(load_spec_from_disk().expect("load spec")));

/* ───── Commands ───── */
#[tauri::command]
fn get_families() -> Result<Vec<String>, String> {
    let spec = SPEC.read().unwrap();
    Ok(spec.families.iter().map(|f| f.start.clone()).collect())
}

#[tauri::command]
fn get_commands() -> Result<Vec<serde_json::Value>, String> {
    let spec = SPEC.read().unwrap();
    let mut out = vec![];

    for fam in &spec.families {
        for cmd in &fam.commands {
            let mut val = serde_json::to_value(cmd).map_err(|e| e.to_string())?;
            if let Some(obj) = val.as_object_mut() {
                obj.insert("familyStart".to_string(), json!(fam.start));
            }
            out.push(val);
        }
    }

    Ok(out)
}

#[tauri::command]
fn append_command(new_cmd: serde_json::Value) -> Result<(), String> {
    let family = new_cmd.get("familyStart").and_then(|v| v.as_str()).ok_or("Missing familyStart")?;
    let path = ensure_user_spec().map_err(|e| e.to_string())?;
    let mut spec: serde_json::Value = serde_json::from_str(&fs::read_to_string(&path).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;

    let families = spec.get_mut("families").and_then(|v| v.as_array_mut()).ok_or("No families array")?;
    let fam = families.iter_mut().find(|f| f.get("start").and_then(|p| p.as_str()) == Some(family))
        .ok_or("Family not found")?;

    let cmds = fam.get_mut("commands").and_then(|v| v.as_array_mut()).ok_or("No commands array")?;
    let letter = new_cmd.get("letter").and_then(|v| v.as_str()).ok_or("Missing letter")?;

    if cmds.iter().any(|c| c.get("letter").and_then(|v| v.as_str()) == Some(letter)) {
        return Err(format!("Command {} already exists in family {}", letter, family));
    }

    let mut to_add = new_cmd.clone();
    to_add.as_object_mut().unwrap().remove("familyStart");
    cmds.push(to_add);

    fs::write(&path, serde_json::to_string_pretty(&spec).unwrap()).map_err(|e| e.to_string())?;
    reload_spec()?;
    Ok(())
}

#[tauri::command]
fn update_command(updated_cmd: serde_json::Value) -> Result<(), String> {
    let letter = updated_cmd.get("letter").and_then(|v| v.as_str()).ok_or("No letter")?;
    let family = updated_cmd.get("familyStart").and_then(|v| v.as_str()).ok_or("Missing familyStart")?;
    let path = ensure_user_spec().map_err(|e| e.to_string())?;
    let mut spec: serde_json::Value = serde_json::from_str(&fs::read_to_string(&path).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;

    let families = spec.get_mut("families").and_then(|v| v.as_array_mut()).ok_or("Bad families")?;
    let fam = families.iter_mut().find(|f| f.get("start").and_then(|p| p.as_str()) == Some(family))
        .ok_or("Family not found")?;

    let cmds = fam.get_mut("commands").and_then(|v| v.as_array_mut()).ok_or("No commands")?;
    if let Some(existing) = cmds.iter_mut().find(|c| c.get("letter").and_then(|v| v.as_str()) == Some(letter)) {
        let mut cleaned = updated_cmd.clone();
        cleaned.as_object_mut().unwrap().remove("familyStart");
        *existing = cleaned;
    } else {
        return Err("Command not found".into());
    }

    fs::write(&path, serde_json::to_string_pretty(&spec).unwrap()).map_err(|e| e.to_string())?;
    reload_spec()?;
    Ok(())
}

#[tauri::command]
fn delete_command(letter: String, family: String) -> Result<(), String> {
    let path = ensure_user_spec().map_err(|e| e.to_string())?;
    let mut spec: serde_json::Value = serde_json::from_str(&fs::read_to_string(&path).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;

    let families = spec.get_mut("families").and_then(|v| v.as_array_mut()).ok_or("Bad spec")?;
    let fam = families.iter_mut().find(|f| f.get("prefix").and_then(|p| p.as_str()) == Some(&family))
        .ok_or("Family not found")?;

    let cmds = fam.get_mut("commands").and_then(|v| v.as_array_mut()).ok_or("No commands")?;
    let before = cmds.len();
    cmds.retain(|c| c.get("letter").and_then(|v| v.as_str()) != Some(&letter));
    if cmds.len() == before {
        return Err("Command not found".into());
    }

    fs::write(&path, serde_json::to_string_pretty(&spec).unwrap()).map_err(|e| e.to_string())?;
    reload_spec()?;
    Ok(())
}

#[tauri::command]
fn decode_frame(frame: String) -> Result<serde_json::Value, String> {
    let cleaned = frame.trim();
    let spec = SPEC.read().unwrap();
    decode(cleaned, &*spec).map_err(|e| e.to_string())
}

#[tauri::command]
fn batch_decode(text: String) -> Result<Vec<serde_json::Value>, String> {
    let spec = SPEC.read().unwrap();
    text.lines()
        .map(|line| decode(line.trim(), &*spec).map_err(|e| e.to_string()))
        .collect()
}

#[tauri::command]
fn reload_spec() -> Result<(), String> {
    let mut spec_lock = SPEC.write().unwrap();
    *spec_lock = load_spec_from_disk().map_err(|e| format!("{:?}", e))?;
    Ok(())
}

/* ───── main ───── */
fn main() {
    tauri::Builder::default()
        .plugin(fs_plugin())
        .invoke_handler(tauri::generate_handler![
            get_families,
            get_commands,
            append_command,
            update_command,
            delete_command,
            decode_frame,
            batch_decode,
            reload_spec
        ])
        .run(tauri::generate_context!())
        .expect("error running tauri app");
}
