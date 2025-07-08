use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use std::{collections::BTreeMap, fs, path::PathBuf, sync::RwLock};
use once_cell::sync::Lazy;
use tauri::command;

/* ──────────── Spec structures ──────────── */
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SpecFile {
    pub families: Vec<Family>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Family {
    pub name: String,
    pub description: Option<String>,
    pub start: String,
    pub terminator: String,
    pub frame_len: u32,
    pub commands: Vec<Command>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Command {
    pub letter: String,
    pub description: Option<String>,
    pub items: Vec<Item>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum Item {
    Group(Group),
    Switch(Switch),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Group {
    pub name: String,
    pub fields: Vec<Field>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Field {
    pub name: String,
    pub len: u32,
    pub base: u8,
    #[serde(rename = "type")]
    pub typ: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Switch {
    pub switch: String,
    pub cases: BTreeMap<String, Variant>,
    #[serde(default)]
    pub default: Option<Variant>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Variant {
    pub description: Option<String>,
    pub groups: Vec<Group>,
}

/* ──────────── Decode error types ──────────── */
#[derive(thiserror::Error, Debug)]
pub enum DecodeError {
    #[error("frame must be fixed length of {expected}, but got {got}")]
    BadLength { expected: u32, got: usize },
    #[error("no family matches frame start")]
    UnknownFamily,
    #[error("terminator must be 1-3 matching characters, got {0}")]
    BadTerminators(usize),
    #[error("unknown command letter '{0}'")]
    UnknownCommand(char),
    #[error("required field '{0}' not available yet (switch)")]
    MissingField(String),
    #[error("field '{0}' is not a number")]
    BadType(String),
    #[error("unknown switch case '{0}' for field '{1}'")]
    UnknownCase(String, String),
    #[error("input too short for field '{field}': need {needed}, got {got}")]
    InputTooShort {
        field: String,
        needed: u32,
        got: u32,
    },
    #[error("invalid spec: {0}")]
    InvalidSpec(String),
    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),
}

/* ──────────── Frame decoder ──────────── */
pub fn decode(frame: &str, spec: &SpecFile) -> Result<Value, DecodeError> {
    // Find the family based on the frame's starting character(s)
    let family = spec
        .families
        .iter()
        .find(|f| !f.start.is_empty() && frame.starts_with(&f.start))
        .ok_or(DecodeError::UnknownFamily)?;

    // Defensive check against a malformed spec that might have been edited manually
    if family.terminator.is_empty() {
        return Err(DecodeError::InvalidSpec("Family terminator is empty in spec.".into()));
    }

    // CORRECTED LOGIC: The total frame length is now always fixed.
    if frame.len() != family.frame_len as usize {
        return Err(DecodeError::BadLength {
            expected: family.frame_len,
            got: frame.len(),
        });
    }

    let term_char = family.terminator.chars().next().unwrap();
    let term_cnt = frame.chars().rev().take_while(|c| *c == term_char).count();
    if !(1..=3).contains(&term_cnt) {
        return Err(DecodeError::BadTerminators(term_cnt));
    }

    // The data payload (body) is now variable length, sliced correctly.
    let body = &frame[family.start.len()..frame.len() - term_cnt];
    let (letter_str, mut cursor) = body.split_at(1);
    let letter = letter_str.chars().next().unwrap();

    let cmd = family
        .commands
        .iter()
        .find(|c| c.letter.chars().next() == Some(letter))
        .ok_or(DecodeError::UnknownCommand(letter))?;

    let mut out = json!({
        "cmd": letter.to_string(),
        "description": cmd.description.clone().unwrap_or_default(),
        "raw": frame
    });
    let mut ctx = Map::new();

    for item in &cmd.items {
        match item {
            Item::Group(g) => {
                out[g.name.clone()] = decode_group(g, &mut cursor, &mut ctx)?;
            }
            Item::Switch(sw) => {
                let disc = ctx.get(&sw.switch)
                    .ok_or_else(|| DecodeError::MissingField(sw.switch.clone()))?
                    .as_u64()
                    .ok_or_else(|| DecodeError::BadType(sw.switch.clone()))?;

                let key = format!("0x{:04X}", disc);
                let var = sw.cases.get(&key).or(sw.default.as_ref())
                    .ok_or_else(|| DecodeError::UnknownCase(key.clone(), sw.switch.clone()))?;

                if let Some(d) = &var.description {
                    out["variant_description"] = Value::String(d.clone());
                }

                for g in &var.groups {
                    out[g.name.clone()] = decode_group(g, &mut cursor, &mut ctx)?;
                }
            }
        }
    }
    Ok(out)
}

fn decode_group(g: &Group, cursor: &mut &str, ctx: &mut Map<String, Value>) -> Result<Value, DecodeError> {
    let mut map = Map::new();
    for f in &g.fields {
        if cursor.len() < f.len as usize {
            return Err(DecodeError::InputTooShort {
                field: f.name.clone(),
                needed: f.len,
                got: cursor.len() as u32,
            });
        }
        let (seg, rest) = cursor.split_at(f.len as usize);
        *cursor = rest;

        let raw_val = slice_val(seg, f.base, &f.typ)?;
        ctx.insert(f.name.clone(), raw_val.clone());

        let entry = match &f.description {
            Some(desc) => json!({ "hex": seg, "value": raw_val, "description": desc }),
            None       => json!({ "hex": seg, "value": raw_val }),
        };
        map.insert(f.name.clone(), entry);
    }
    Ok(Value::Object(map))
}

fn slice_val(slice: &str, base: u8, typ: &str) -> Result<Value, DecodeError> {
    let n = u32::from_str_radix(slice, base.into())?;
    Ok(match typ {
        "bool" => Value::Bool(n != 0),
        "u8" => Value::from(n as u8),
        "u16" => Value::from(n as u16),
        "u32" => Value::from(n),
        _ => Value::from(n),
    })
}
