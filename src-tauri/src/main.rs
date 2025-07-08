#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// ────────────────────────────────────────────────────────────────
// FULL main.rs — multi‑family backend with legacy default‑spec support
// ────────────────────────────────────────────────────────────────

mod decoder;

use decoder::{decode, Command, Family, SpecFile};
use once_cell::sync::Lazy;
use serde_json::Value;
use std::{fs, path::PathBuf, sync::RwLock};
use tauri::{Manager};
use tauri_plugin_fs::init as fs_plugin;
use dirs_next::config_dir;

/* ─────────── Constants & Embedded Default ─────────── */
const APP_DIR: &str = "EZFrameDecoder";
const USER_SPEC_FILE: &str = "spec_override.json";
// The file bundled at compile‑time — still called spec_full.json but in new schema
const DEFAULT_SPEC: &str = include_str!("../resources/spec_full.json");

/* ─────────── ensure + load helpers ─────────── */
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

fn load_spec_from_disk() -> Result<SpecFile, Box<dyn std::error::Error>> {
    let path = ensure_user_spec()?;
    let text = fs::read_to_string(&path)?;
    let spec = serde_json::from_str::<SpecFile>(&text)?;
    Ok(spec)
}

/* ─────────── live, hot‑reloadable spec instance ─────────── */
static SPEC: Lazy<RwLock<SpecFile>> =
    Lazy::new(|| RwLock::new(load_spec_from_disk().expect("load spec")));

/* ─────────── internal helpers ─────────── */
fn reload_spec() -> Result<(), String> {
    let mut spec_lock = SPEC.write().unwrap();
    *spec_lock = load_spec_from_disk().map_err(|e| e.to_string())?;
    Ok(())
}

fn mutating_spec<F>(op: F) -> Result<(), String>
where
    F: FnOnce(&mut SpecFile) -> Result<(), String>,
{
    let path = ensure_user_spec().map_err(|e| e.to_string())?;
    let mut spec: SpecFile = serde_json::from_str(&fs::read_to_string(&path).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    op(&mut spec)?;
    fs::write(&path, serde_json::to_string_pretty(&spec).unwrap()).map_err(|e| e.to_string())?;
    reload_spec()?;
    Ok(())
}

fn clean_frame(line: &str) -> &str {
    line.split_whitespace().next().unwrap_or("")
}

/// Validates a family's core properties.
fn validate_family_properties(fam: &Family) -> Result<(), String> {
    if fam.name.trim().is_empty() {
        return Err("Family name cannot be empty.".into());
    }
    if fam.start.trim().is_empty() {
        return Err("Family start character cannot be empty.".into());
    }
    if fam.terminator.trim().is_empty() {
        return Err("Family terminator cannot be empty.".into());
    }
    if fam.start == fam.terminator {
        return Err("Family start and terminator cannot be the same.".into());
    }
    Ok(())
}

/* ─────────── Tauri commands ─────────── */
#[tauri::command]
fn decode_frame(frame: String) -> Result<Value, String> {
    let trimmed = clean_frame(&frame);
    if trimmed.is_empty() {
        return Err("Empty line".into());
    }
    let spec = SPEC.read().unwrap();
    decode(trimmed, &*spec).map_err(|e| e.to_string())
}

#[tauri::command]
fn batch_decode(text: String) -> Result<Vec<Value>, String> {
    let spec = SPEC.read().unwrap();
    text.lines()
        .filter_map(|raw| {
            let cleaned = clean_frame(raw.trim());
            if cleaned.is_empty() {
                None
            } else {
                Some(decode(cleaned, &*spec).map_err(|e| e.to_string()))
            }
        })
        .collect()
}

// ─────────── Family CRUD ───────────
#[tauri::command]
fn get_families() -> Result<Vec<Family>, String> {
    Ok(SPEC.read().unwrap().families.clone())
}

#[tauri::command]
fn create_family(fam: Family) -> Result<(), String> {
    mutating_spec(|spec| {
        validate_family_properties(&fam)?;
        if spec.families.iter().any(|f| f.start == fam.start) {
            return Err("A family with that start character already exists".into());
        }
        spec.families.push(fam);
        Ok(())
    })
}

#[tauri::command]
fn update_family(original_start: String, fam: Family) -> Result<(), String> {
    mutating_spec(|spec| {
        validate_family_properties(&fam)?;
        // If the start character was changed, check if the new one conflicts with any *other* family.
        if original_start != fam.start && spec.families.iter().any(|f| f.start == fam.start) {
            return Err("Another family with the new start character already exists.".into());
        }

        let slot = spec
            .families
            .iter_mut()
            .find(|f| f.start == original_start)
            .ok_or("Family to update not found (using original start char).")?;

        *slot = fam;
        Ok(())
    })
}

#[tauri::command]
fn delete_family(start: String) -> Result<(), String> {
    mutating_spec(|spec| {
        let before = spec.families.len();
        spec.families.retain(|f| f.start != start);
        if spec.families.len() == before {
            return Err("Family not found".into());
        }
        Ok(())
    })
}

// ─────────── Command CRUD (scoped to family) ───────────
#[tauri::command]
fn get_commands(family_start: String) -> Result<Vec<Command>, String> {
    let spec = SPEC.read().unwrap();
    let fam = spec
        .families
        .iter()
        .find(|f| f.start == family_start)
        .ok_or("Family not found")?;
    Ok(fam.commands.clone())
}

#[tauri::command]
fn append_command(family_start: String, cmd: Command) -> Result<(), String> {
    mutating_spec(|spec| {
        let fam = spec
            .families
            .iter_mut()
            .find(|f| f.start == family_start)
            .ok_or("Family not found")?;
        if cmd.letter.trim().is_empty() {
            return Err("Command letter cannot be empty.".into());
        }
        if fam.commands.iter().any(|c| c.letter == cmd.letter) {
            return Err("Command letter already present in this family".into());
        }
        fam.commands.push(cmd);
        Ok(())
    })
}

#[tauri::command]
fn update_command(family_start: String, original_letter: String, cmd: Command) -> Result<(), String> {
    mutating_spec(|spec| {
        let fam = spec
            .families
            .iter_mut()
            .find(|f| f.start == family_start)
            .ok_or("Family not found")?;

        if cmd.letter.trim().is_empty() {
            return Err("Command letter cannot be empty.".into());
        }

        // If the letter was changed, check for conflicts with other commands in the same family.
        if original_letter != cmd.letter && fam.commands.iter().any(|c| c.letter == cmd.letter) {
            return Err("Another command with that letter already exists in this family.".into());
        }

        let slot = fam
            .commands
            .iter_mut()
            .find(|c| c.letter == original_letter)
            .ok_or("Command to update not found (using original letter).")?;

        *slot = cmd;
        Ok(())
    })
}

#[tauri::command]
fn delete_command(family_start: String, letter: String) -> Result<(), String> {
    mutating_spec(|spec| {
        let fam = spec
            .families
            .iter_mut()
            .find(|f| f.start == family_start)
            .ok_or("Family not found")?;
        let before = fam.commands.len();
        fam.commands.retain(|c| c.letter != letter);
        if fam.commands.len() == before {
            return Err("Command not found".into());
        }
        Ok(())
    })
}

/* ─────────── Application entry point ─────────── */
fn main() {
    tauri::Builder::default()
        .plugin(fs_plugin())
        .invoke_handler(tauri::generate_handler![
            // Decode
            decode_frame,
            batch_decode,
            // Family CRUD
            get_families,
            create_family,
            update_family,
            delete_family,
            // Command CRUD
            get_commands,
            append_command,
            update_command,
            delete_command,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}
