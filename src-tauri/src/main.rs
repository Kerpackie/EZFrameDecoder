#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod decoder;

use decoder::{decode, Command, Family, SpecFile};
use dirs_next::config_dir;
use once_cell::sync::Lazy;
use serde_json::Value;
use std::{fs, path::PathBuf, sync::RwLock};
use tauri::Manager;

/* ─────────── Constants & Embedded Default ─────────── */
const APP_DIR: &str = "EZFrameDecoder";
const USER_SPEC_FILE: &str = "spec_override.ezspec";
const DEFAULT_SPEC: &str = include_str!("../resources/spec_full.ezspec");

/* ─────────── State Management ─────────── */

/// Holds the application's current state, including the loaded spec and its origin path.
#[derive(Clone)]
struct AppState {
    spec: SpecFile,
    path: PathBuf, // The path of the currently loaded spec file.
}

/// The global, thread-safe state of the application.
static STATE: Lazy<RwLock<AppState>> = Lazy::new(|| {
    RwLock::new(
        load_default_spec_or_heal()
            .expect("Fatal error: Could not load or heal the default spec file on startup."),
    )
});

/* ─────────── Core Loading & Healing Logic ─────────── */

/// Gets the path to the default user override file, creating it from the bundled
/// spec if it doesn't exist.
fn get_default_spec_path() -> Result<PathBuf, String> {
    let path = config_dir()
        .ok_or_else(|| "Could not find config directory".to_string())?
        .join(APP_DIR)
        .join(USER_SPEC_FILE);

    if !path.exists() {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        fs::write(&path, DEFAULT_SPEC).map_err(|e| e.to_string())?;
        println!("★ Copied bundled default spec to {}", path.display());
    }
    Ok(path)
}

/// Loads a spec from a specific file path.
fn load_spec_from_path(path: &PathBuf) -> Result<SpecFile, String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read spec file at {}: {}", path.display(), e))?;
    serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse spec file at {}: {}", path.display(), e))
}

/// Loads the default `spec_override.ezspec`. If it's corrupt, it "heals" it by
/// overwriting it with the bundled default.
fn load_default_spec_or_heal() -> Result<AppState, String> {
    let path = get_default_spec_path()?;
    let spec = match load_spec_from_path(&path) {
        Ok(spec) => {
            println!("Successfully loaded default user spec: {}", path.display());
            spec
        }
        Err(e) => {
            eprintln!("Corrupt default spec file found. Healing. Error: {}", e);
            fs::write(&path, DEFAULT_SPEC).map_err(|e| e.to_string())?;
            serde_json::from_str(DEFAULT_SPEC)
                .map_err(|e| format!("FATAL: Failed to parse bundled default spec: {}", e))?
        }
    };
    Ok(AppState { spec, path })
}

/* ─────────── Internal Helpers ─────────── */

/// A generic function to perform mutations. It locks the current state,
/// applies an operation, and then saves the result back to the original file path.
fn mutating_spec<F>(op: F) -> Result<(), String>
where
    F: FnOnce(&mut SpecFile) -> Result<(), String>,
{
    // 1. Get a write lock on the global state.
    let mut state_lock = STATE.write().unwrap();

    // 2. Apply the mutation directly to the `spec` within the locked state.
    op(&mut state_lock.spec)?;

    // 3. Serialize the *entire*, now-mutated, in-memory spec to a JSON string.
    let new_content = serde_json::to_string_pretty(&state_lock.spec)
        .map_err(|e| format!("Failed to serialize spec: {}", e))?;

    // 4. Write the new content back to the *original path* stored in the state.
    fs::write(&state_lock.path, &new_content)
        .map_err(|e| format!("Failed to write spec to file: {}", e))?;

    println!("Successfully mutated and persisted spec at {}", state_lock.path.display());
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

/* ─────────── Tauri Commands ─────────── */

/// Loads a spec from a user-selected file path and sets it as the active spec.
#[tauri::command]
fn load_spec(path_str: String) -> Result<(), String> {
    let path = PathBuf::from(path_str);
    let spec = load_spec_from_path(&path)?;
    let mut state_lock = STATE.write().unwrap();
    *state_lock = AppState { spec, path };
    Ok(())
}

/// Resets the active spec to the default `spec_override.ezspec`.
#[tauri::command]
fn reset_spec_to_default() -> Result<(), String> {
    let new_state = load_default_spec_or_heal()?;
    let mut state_lock = STATE.write().unwrap();
    *state_lock = new_state;
    Ok(())
}

/// Returns the currently loaded spec families.
#[tauri::command]
fn get_families() -> Result<Vec<Family>, String> {
    Ok(STATE.read().unwrap().spec.families.clone())
}

/// Returns the commands for a specific family from the currently loaded spec.
#[tauri::command]
fn get_commands(family_start: String) -> Result<Vec<Command>, String> {
    let state = STATE.read().unwrap();
    let fam = state.spec
        .families
        .iter()
        .find(|f| f.start == family_start)
        .ok_or("Family not found")?;
    Ok(fam.commands.clone())
}

/// Returns the entire content of the currently loaded spec file.
#[tauri::command]
fn get_spec_content() -> Result<String, String> {
    let state = STATE.read().unwrap();
    serde_json::to_string_pretty(&state.spec).map_err(|e| format!("Failed to serialize spec: {}", e))
}

/// Decodes a single frame using the currently active spec.
#[tauri::command]
fn decode_frame(frame: String) -> Result<Value, String> {
    let trimmed = clean_frame(&frame);
    if trimmed.is_empty() {
        return Err("Empty line".into());
    }
    let state = STATE.read().unwrap();
    decode(trimmed, &state.spec).map_err(|e| e.to_string())
}

/// Decodes multiple frames using the currently active spec.
#[tauri::command]
fn batch_decode(text: String) -> Result<Vec<Value>, String> {
    let state = STATE.read().unwrap();
    text.lines()
        .filter_map(|raw| {
            let cleaned = clean_frame(raw.trim());
            if cleaned.is_empty() {
                None
            } else {
                Some(decode(cleaned, &state.spec).map_err(|e| e.to_string()))
            }
        })
        .collect()
}

// ─────────── CRUD Commands (now use mutating_spec) ───────────

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
fn update_command(
    family_start: String,
    original_letter: String,
    cmd: Command,
) -> Result<(), String> {
    mutating_spec(|spec| {
        let fam = spec
            .families
            .iter_mut()
            .find(|f| f.start == family_start)
            .ok_or("Family not found")?;
        if cmd.letter.trim().is_empty() {
            return Err("Command letter cannot be empty.".into());
        }
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
            return Err("Family not found".into());
        }
        Ok(())
    })
}

/* ─────────── Application Entry Point ─────────── */
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            // Spec file management
            load_spec,
            reset_spec_to_default,
            get_spec_content,
            // Decode
            decode_frame,
            batch_decode,
            // Family & Command Read
            get_families,
            get_commands,
            // Family & Command Write (mutations)
            create_family,
            update_family,
            delete_family,
            append_command,
            update_command,
            delete_command,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}
