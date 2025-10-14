/*
Module: core::id_policy
Purpose: Small, pure utility for entry ID selection.
Public API: choose_id(guid, link, content_hash) -> String
Invariants: Precedence = guid > link > content_hash; outputs stable ASCII.
Notes: No I/O; no hashing here (hash must be provided).
*/
#![forbid(unsafe_code)]

/// Choose a stable entry identifier by precedence:
/// 1) Non-empty `guid` => return as-is
/// 2) Else non-empty `link` => return as-is
/// 3) Else fallback to `content_hash` (must be provided, non-empty)
pub fn choose_id(guid: Option<&str>, link: Option<&str>, content_hash: &str) -> String {
    let g = guid.and_then(non_empty);
    if let Some(g) = g {
        return g.to_owned();
    }

    let l = link.and_then(non_empty);
    if let Some(l) = l {
        return l.to_owned();
    }

    content_hash.to_owned()
}

fn non_empty(s: &str) -> Option<&str> {
    let t = s.trim();
    if t.is_empty() {
        None
    } else {
        Some(t)
    }
}
