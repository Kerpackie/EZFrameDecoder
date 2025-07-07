//! Pure Rust decoder for EZ-Frame
//!
//! * `SpecFile` – top-level spec (now groups commands by *family* start byte)
//! * `decode(frame, &spec)` → `serde_json::Value`

use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use std::collections::BTreeMap;

/* ─────────── protocol constants ─────────── */
pub const FRAME_LEN: usize = 23;

/* ────────────── spec structs ────────────── */

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecFile {
    /// List of frame families: one per different start byte
    pub families: Vec<Family>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Family {
    /// Single ASCII start byte, e.g. "<"  or "["  etc.
    pub start: String,
    /// Commands that live under this start byte
    pub commands: Vec<Command>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    /// “A” .. “Z” – kept as short `String` for JSON friendliness
    pub letter: String,
    pub description: Option<String>,
    pub items: Vec<Item>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Item {
    Group(Group),
    Switch(Switch),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub name: String,
    pub fields: Vec<Field>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    pub len: usize,
    pub base: u8,
    #[serde(rename = "type")]
    pub typ: String,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Switch {
    pub switch: String,                       // discriminant field name
    pub cases: BTreeMap<String, Variant>,     // keyed like "0x1700"
    #[serde(default)]
    pub default: Option<Variant>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Variant {
    #[serde(default)]
    pub description: Option<String>,
    pub groups: Vec<Group>,
}

/* ────────────── error type ─────────────── */

#[derive(thiserror::Error, Debug)]
pub enum DecodeError {
    #[error("frame must be {FRAME_LEN} bytes, got {0}")]
    BadLength(usize),
    #[error("unknown family start byte '{0}'")]
    UnknownFamily(char),
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
        needed: usize,
        got: usize,
    },
    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),
}

/* ───────── helper: numeric slice → Value ───────── */
fn slice_val(slice: &str, base: u8, typ: &str) -> Result<Value, DecodeError> {
    let n = u32::from_str_radix(slice, base.into())?;
    Ok(match typ {
        "bool" => Value::Bool(n != 0),
        "u8" | "number" if slice.len() == 2 => Value::from(n as u8),
        "u16" if slice.len() == 4 => Value::from(n as u16),
        "u24" if slice.len() == 6 => Value::from(n as u32),
        "u32" if slice.len() == 8 => Value::from(n),
        _ => Value::from(n),
    })
}

/* ───────── decode a single group ───────── */
fn decode_group(
    g: &Group,
    cursor: &mut &str,
    ctx: &mut Map<String, Value>,
) -> Result<Value, DecodeError> {
    let mut map = Map::new();

    for f in &g.fields {
        if cursor.len() < f.len {
            return Err(DecodeError::InputTooShort {
                field: f.name.clone(),
                needed: f.len,
                got: cursor.len(),
            });
        }

        let (seg, rest) = cursor.split_at(f.len);
        *cursor = rest;

        let raw_val = slice_val(seg, f.base, &f.typ)?;
        ctx.insert(f.name.clone(), raw_val.clone());

        let entry = match &f.description {
            Some(desc) => json!({
                "hex": seg,
                "value": raw_val,
                "description": desc
            }),
            None => json!({ "hex": seg, "value": raw_val }),
        };
        map.insert(f.name.clone(), entry);
    }

    Ok(Value::Object(map))
}

/* ─────────── main decode API ─────────── */

pub fn decode(frame: &str, spec: &SpecFile) -> Result<Value, DecodeError> {
    if frame.len() != FRAME_LEN {
        return Err(DecodeError::BadLength(frame.len()));
    }

    let start_char = frame.chars().next().unwrap(); // 1st byte
    let family = spec
        .families
        .iter()
        .find(|f| f.start.chars().next().unwrap() == start_char)
        .ok_or(DecodeError::UnknownFamily(start_char))?;

    let body = &frame[1..FRAME_LEN];
    let (letter_str, mut cursor) = body.split_at(1);
    let letter = letter_str.chars().next().unwrap();

    let cmd = family
        .commands
        .iter()
        .find(|c| c.letter == letter.to_string())
        .ok_or(DecodeError::UnknownCommand(letter))?;

    let mut out = json!({
        "family": family.start,
        "cmd": letter.to_string(),
        "description": cmd.description.clone().unwrap_or_default(),
        "raw": frame
    });

    let mut ctx = Map::new();

    for item in &cmd.items {
        match item {
            Item::Group(g) => {
                let val = decode_group(g, &mut cursor, &mut ctx)?;
                out[g.name.clone()] = val;
            }
            Item::Switch(sw) => {
                let disc = ctx
                    .get(&sw.switch)
                    .ok_or_else(|| DecodeError::MissingField(sw.switch.clone()))?
                    .as_u64()
                    .ok_or_else(|| DecodeError::BadType(sw.switch.clone()))?;
                let key = format!("0x{:04X}", disc);

                let variant = sw
                    .cases
                    .get(&key)
                    .or(sw.default.as_ref())
                    .ok_or_else(|| DecodeError::UnknownCase(key.clone(), sw.switch.clone()))?;

                if let Some(d) = &variant.description {
                    out["variant_description"] = Value::String(d.clone());
                }
                for g in &variant.groups {
                    let val = decode_group(g, &mut cursor, &mut ctx)?;
                    out[g.name.clone()] = val;
                }
            }
        }
    }

    Ok(out)
}
