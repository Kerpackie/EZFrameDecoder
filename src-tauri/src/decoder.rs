//! Pure Rust decoder
//!
//! * `SpecFile` – the JSON-driven command layout
//! * `decode(frame, &spec)` – returns a `serde_json::Value` tree
//!
//! Supports:
//!   • 23-byte ASCII frames (`<` … `>` / `>>` / `>>>`)  
//!   • Per-command groups and switches (variants)  
//!   • Per-field **description**, which the caller may surface in the UI  
//!
//! If a field has `"description": "..."`, the returned JSON is
//! ```json
//! "Opcode": {
//!   "value": 20483,
//!   "description": "Operation selector"
//! }
//! ```
//! Otherwise it is a plain JSON value.

use serde::Deserialize;
use serde_json::{json, Map, Value};
use std::collections::BTreeMap;

/* ───────────── protocol constants ───────────── */
pub const FRAME_LEN: usize = 23;

/* ─────────────── spec structures ────────────── */

#[derive(Debug, Deserialize)]
pub struct SpecFile {
    pub framing: Framing,
    pub commands: Vec<Command>,
}
#[derive(Debug, Deserialize)]
pub struct Framing {
    pub start: String,
}

#[derive(Debug, Deserialize)]
pub struct Command {
    pub letter: char,
    pub description: Option<String>,
    #[serde(rename = "items")]
    pub items: Vec<Item>,
}

/* item can be a Group or a Switch  */
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Item {
    Group(Group),
    Switch(Switch),
}

/* group = named collection of fields */
#[derive(Debug, Deserialize)]
pub struct Group {
    pub name: String,
    pub fields: Vec<Field>,
}

#[derive(Debug, Deserialize)]
pub struct Field {
    pub name: String,
    pub len: usize,
    pub base: u8,
    #[serde(rename = "type")]
    pub typ: String,
    #[serde(default)]
    pub description: Option<String>,
}

/* switch → branch on already-parsed discriminant */
#[derive(Debug, Deserialize)]
pub struct Variant {
    #[serde(default)]
    pub description: Option<String>,
    pub groups: Vec<Group>,
}

#[derive(Debug, Deserialize)]
pub struct Switch {
    pub switch: String, /* field name */
    pub cases: BTreeMap<String, Variant>,
    #[serde(default)]
    pub default: Option<Variant>,
}

/* ────────────── error handling ─────────────── */

#[derive(thiserror::Error, Debug)]
pub enum DecodeError {
    #[error("frame must be {FRAME_LEN} bytes, got {0}")]
    BadLength(usize),
    #[error("frame must start with '<'")]
    BadStart,
    #[error("terminator must be 1-3 '>' characters, got {0}")]
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
        needed: usize,
        got: usize,
    },
    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),
}

/* ───────────── helper: decode numeric slice ───────────── */
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

/* ───────────── decode a single group ───────────── */
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
                "hex": seg,        // raw slice exactly as seen in the frame
                "value": raw_val,
                "description": desc
            }),
            None => json!({
                "hex": seg,
                "value": raw_val
            }),
        };
        map.insert(f.name.clone(), entry);
    }

    Ok(Value::Object(map))
}

/* ───────────── public decode API ───────────── */

pub fn decode(frame: &str, spec: &SpecFile) -> Result<Value, DecodeError> {
    /* a. sanity checks */
    if frame.len() != FRAME_LEN {
        return Err(DecodeError::BadLength(frame.len()));
    }
    if !frame.starts_with('<') {
        return Err(DecodeError::BadStart);
    }
    let term_cnt = frame.chars().rev().take_while(|c| *c == '>').count();
    if term_cnt == 0 || term_cnt > 3 {
        return Err(DecodeError::BadTerminators(term_cnt));
    }

    /* b. slice frame */
    let body = &frame[1..FRAME_LEN - term_cnt];
    let (letter_str, mut cursor) = body.split_at(1);
    let letter = letter_str.chars().next().unwrap();

    /* c. lookup command */
    let cmd = spec
        .commands
        .iter()
        .find(|c| c.letter == letter)
        .ok_or(DecodeError::UnknownCommand(letter))?;

    let mut out = json!({
        "cmd": letter.to_string(),
        "description": cmd.description.clone().unwrap_or_default(),
        "raw": frame
    });
    let mut ctx = Map::new(); // parsed fields cache

    /* d. walk items in order */
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
                let var = sw
                    .cases
                    .get(&key)
                    .or(sw.default.as_ref())
                    .ok_or_else(|| DecodeError::UnknownCase(key.clone(), sw.switch.clone()))?;

                if let Some(d) = &var.description {
                    out["variant_description"] = Value::String(d.clone());
                }
                for g in &var.groups {
                    let val = decode_group(g, &mut cursor, &mut ctx)?;
                    out[g.name.clone()] = val;
                }
            }
        }
    }
    /* any leftover cursor chars = padding → ignored */
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn make_spec() -> SpecFile {
        serde_json::from_str(r#"
    {
        "framing": { "start": "<" },
        "commands": [
            {
                "letter": "A",
                "description": "Test command",
                "items": [
                    {
                        "name": "Header",
                        "fields": [
                            { "name": "Opcode", "len": 4, "base": 10, "type": "u16", "description": "Operation selector" },
                            { "name": "Flag", "len": 1, "base": 10, "type": "bool" }
                        ]
                    }
                ]
            },
            {
                "letter": "B",
                "description": "Switch command",
                "items": [
                    {
                        "name": "Header",
                        "fields": [
                            { "name": "SwitchField", "len": 2, "base": 16, "type": "u8" }
                        ]
                    },
                    {
                        "switch": "SwitchField",
                        "cases": {
                            "0x0001": {
                                "description": "Case 1",
                                "groups": [
                                    {
                                        "name": "Case1Group",
                                        "fields": [
                                            { "name": "Value", "len": 2, "base": 10, "type": "u8" }
                                        ]
                                    }
                                ]
                            }
                        },
                        "default": {
                            "description": "Default case",
                            "groups": [
                                {
                                    "name": "DefaultGroup",
                                    "fields": [
                                        { "name": "Value", "len": 2, "base": 10, "type": "u8" }
                                    ]
                                }
                            ]
                        }
                    }
                ]
            }
        ]
    }
    "#).unwrap()
    }

    #[test]
    fn decodes_valid_frame_with_description() {
        let spec = make_spec();
        // Frame: <A20481 0           >>> (23 chars)
        let frame = "<A20481 0           >>>";
        let result = decode(frame, &spec).unwrap();
        assert_eq!(result["cmd"], "A");
        assert_eq!(result["description"], "Test command");
        assert_eq!(result["Header"]["Opcode"]["value"], 2048);
        assert_eq!(
            result["Header"]["Opcode"]["description"],
            "Operation selector"
        );
        assert_eq!(result["Header"]["Flag"], true);
    }

    #[test]
    fn returns_error_for_invalid_length() {
        let spec = make_spec();
        // Too short (10 chars)
        let frame = "<A12345>>>";
        let err = decode(frame, &spec).unwrap_err();
        matches!(err, DecodeError::BadLength(_));
    }

    #[test]
    fn returns_error_for_invalid_start_char() {
        let spec = make_spec();
        // Not starting with '<'
        let frame = "XA20481 0           >>>";
        let err = decode(frame, &spec).unwrap_err();
        matches!(err, DecodeError::BadStart);
    }

    #[test]
    fn returns_error_for_invalid_terminator_count() {
        let spec = make_spec();
        // Only one '>' at end (should be 1-3, but not enough padding)
        let frame = "<A20481 0           >  ";
        let err = decode(frame, &spec).unwrap_err();
        matches!(err, DecodeError::BadTerminators(_));
    }

    #[test]
    fn returns_error_for_unknown_command() {
        let spec = make_spec();
        // Command letter 'Z' is not in spec
        let frame = "<Z04811 0           >>>";
        let err = decode(frame, &spec).unwrap_err();
        matches!(err, DecodeError::UnknownCommand('Z'));
    }

    #[test]
    fn decodes_switch_case_and_default() {
        let spec = make_spec();
        // Case 1: SwitchField = 01 (hex), matches case
        // <B0199              >>> (23 chars)
        let frame_case = "<B0199              >>>"; // 23 chars
        let result_case = decode(frame_case, &spec).unwrap();
        assert_eq!(result_case["variant_description"], "Case 1");
        assert_eq!(result_case["Case1Group"]["Value"], 99);

        // Default: SwitchField = 02 (hex), no case, uses default
        // <B0288              >>> (23 chars)
        let frame_default = "<B0288              >>>"; // 23 chars
        let result_default = decode(frame_default, &spec).unwrap();
        assert_eq!(result_default["variant_description"], "Default case");
        assert_eq!(result_default["DefaultGroup"]["Value"], 88);
    }

    #[test]
    fn returns_error_for_missing_switch_field() {
        let spec = make_spec();
        // Remove SwitchField from frame (too short)
        let frame = "<B  99                >>>";
        let err = decode(frame, &spec).unwrap_err();
        matches!(err, DecodeError::InputTooShort { .. });
    }

    #[test]
    fn returns_error_for_unknown_switch_case_without_default() {
        let mut spec = make_spec();
        // Remove default from switch
        if let Item::Switch(sw) = &mut spec.commands[1].items[1] {
            sw.default = None;
        }
        let frame = "<B03 77              >>>";
        let err = decode(frame, &spec).unwrap_err();
        matches!(err, DecodeError::UnknownCase(_, _));
    }
}
