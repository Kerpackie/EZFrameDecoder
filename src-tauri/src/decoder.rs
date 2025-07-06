use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use std::collections::BTreeMap;

/* ───── protocol constants ───── */
pub const FRAME_LEN: usize = 23;

/* ───────── spec structures ───────── */

#[derive(Debug, Deserialize)]
pub struct SpecFile {
    pub framing: Framing,
    pub commands: Vec<Command>,
}
#[derive(Debug, Deserialize)]
pub struct Framing {
    pub start: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Command {
    pub letter: String,                       // char → String
    pub description: Option<String>,
    #[serde(rename = "items")]
    pub items: Vec<Item>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Item {
    Group(Group),
    Switch(Switch),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Group {
    pub name: String,
    pub fields: Vec<Field>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Field {
    pub name: String,
    pub len: u32,                             // usize → u32
    pub base: u8,
    #[serde(rename = "type")]
    pub typ: String,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Variant {
    #[serde(default)]
    pub description: Option<String>,
    pub groups: Vec<Group>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Switch {
    pub switch: String,
    pub cases: BTreeMap<String, Variant>,
    #[serde(default)]
    pub default: Option<Variant>,
}

/* ───────── errors ───────── */

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
        needed: u32,                          // usize → u32
        got: u32,                             // usize → u32
    },
    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),
}

/* ───────── helpers ───────── */

fn slice_val(slice: &str, base: u8, typ: &str) -> Result<Value, DecodeError> {
    let n = u32::from_str_radix(slice, base.into())?;
    Ok(match typ {
        "bool" => Value::Bool(n != 0),
        "u8"  => Value::from(n as u8),
        "u16" => Value::from(n as u16),
        "u32" => Value::from(n),
        _     => Value::from(n),
    })
}

fn decode_group(
    g: &Group,
    cursor: &mut &str,
    ctx: &mut Map<String, Value>,
) -> Result<Value, DecodeError> {
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

/* ───────── public API ───────── */

pub fn decode(frame: &str, spec: &SpecFile) -> Result<Value, DecodeError> {
    if frame.len() != FRAME_LEN { return Err(DecodeError::BadLength(frame.len())); }
    if !frame.starts_with('<')  { return Err(DecodeError::BadStart); }

    let term_cnt = frame.chars().rev().take_while(|c| *c == '>').count();
    if !(1..=3).contains(&term_cnt) { return Err(DecodeError::BadTerminators(term_cnt)); }

    let body = &frame[1..FRAME_LEN - term_cnt];
    let (letter_str, mut cursor) = body.split_at(1);
    let letter = letter_str.chars().next().unwrap();

    let cmd = spec.commands
        .iter()
        .find(|c| c.letter.chars().next() == Some(letter))  // String compare
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
                let var = sw.cases.get(&key)
                    .or(sw.default.as_ref())
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
