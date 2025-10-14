#!/usr/bin/env python3
import sys, re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
RS = list(ROOT.glob("crates/**/*.rs"))

MISSING = []
HDR_RE = re.compile(r"^\s*/\*\s*Module:\s*.+", re.MULTILINE)

for f in RS:
    text = f.read_text(encoding="utf-8", errors="ignore")
    if not HDR_RE.search(text):
        MISSING.append(str(f.relative_to(ROOT)))

if MISSING:
    print("ERROR: Missing required header in these files:")
    for p in MISSING:
        print(f" - {p}")
    sys.exit(1)
else:
    print("OK: All Rust files contain the required header.")
