#!/usr/bin/env python3
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
THRESH_WARN = 200
THRESH_ERR = 300


def count_loc(path: Path) -> int:
    lines = path.read_text(encoding="utf-8", errors="ignore").splitlines()
    # Strip the top header block if present (between /* and */ at file start)
    i = 0
    if i < len(lines) and lines[i].lstrip().startswith("/*"):
        # consume until closing */
        while i < len(lines) and "*/" not in lines[i]:
            i += 1
        if i < len(lines):
            i += 1  # consume the line with */
    # Count non-empty, non-// comment lines after header
    loc = 0
    for ln in lines[i:]:
        s = ln.strip()
        if not s:
            continue
        if s.startswith("//"):
            continue
        loc += 1
    return loc


warns = []
errs = []

for f in ROOT.glob("crates/**/*.rs"):
    # Skip test directories
    p = f.as_posix()
    if "/tests/" in p:
        continue
    loc = count_loc(f)
    if loc >= THRESH_ERR:
        errs.append((loc, p))
    elif loc > THRESH_WARN:
        warns.append((loc, p))

if warns:
    print("WARN: Files over 200 LOC (excluding header/tests):")
    for loc, p in sorted(warns, reverse=True):
        print(f" - {p}: {loc} LOC")

if errs:
    print("\nERROR: Files at or above 300 LOC (excluding header/tests):")
    for loc, p in sorted(errs, reverse=True):
        print(f" - {p}: {loc} LOC")
    sys.exit(1)

print("\nOK: LOC within limits.")
