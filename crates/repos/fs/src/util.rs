//// File: crates/repos/fs/src/util.rs
//// Role: Safe ID escaping and atomic write utilities shared by repo code.

use std::fs;
use std::io::Write;
use std::path::Path;

/// Escape an id into a filesystem-safe string:
/// - ASCII alnum, '-' and '_' pass through
/// - everything else becomes _XX (lower-hex)
pub fn escape_id(id: &str) -> String {
    let mut out = String::with_capacity(id.len());
    for b in id.bytes() {
        let c = b as char;
        if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
            out.push(c);
        } else {
            out.push('_');
            out.push(hex(b >> 4));
            out.push(hex(b & 0x0F));
        }
    }
    out
}

/// Best-effort reverse of escape_id (for presentation only).
pub fn unescape_id(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out: Vec<u8> = Vec::with_capacity(bytes.len());
    let mut i = 0usize;
    while i < bytes.len() {
        if bytes[i] == b'_' && i + 2 < bytes.len() {
            if let (Some(h), Some(l)) = (unhex(bytes[i + 1]), unhex(bytes[i + 2])) {
                out.push((h << 4) | l);
                i += 3;
                continue;
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).to_string()
}

fn hex(n: u8) -> char {
    match n {
        0..=9 => (b'0' + n) as char,
        10..=15 => (b'a' + (n - 10)) as char,
        _ => '?',
    }
}

fn unhex(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(10 + b - b'a'),
        b'A'..=b'F' => Some(10 + b - b'A'),
        _ => None,
    }
}

/// Atomic write of a JSON value to `path`.
pub fn write_json_atomic(path: &Path, value: &serde_json::Value) -> std::io::Result<()> {
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;
    }
    let tmp = path.with_extension("json.tmp");
    {
        let mut f = fs::File::create(&tmp)?;
        let s = serde_json::to_string_pretty(value)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        f.write_all(s.as_bytes())?;
        f.sync_all()?;
    }
    fs::rename(tmp, path)
}

/// Atomic write of UTF-8 text to `path`.
pub fn write_text_atomic(path: &Path, text: &str) -> std::io::Result<()> {
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;
    }
    let tmp = path.with_extension("tmp");
    {
        let mut f = fs::File::create(&tmp)?;
        f.write_all(text.as_bytes())?;
        f.sync_all()?;
    }
    fs::rename(tmp, path)
}

