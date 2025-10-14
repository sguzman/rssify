# Repository Backends (Selection Surface)

Phase 1 (MVP):
- fs:<root>        Filesystem repository rooted at <root> (absolute or relative)

Phase 2:
- sqlite:<dsn>     SQLite repository using <dsn> (file path or URI)

Parsing rules:
- Case-insensitive backend prefix (fs, sqlite)
- Format: "<kind>:<target>"
- Reject empty targets
- No I/O or existence checks at parse time
- Backward-compatible: adding new kinds must not break existing specs

Examples:
- fs:/var/lib/rssify
- fs:./data
- sqlite:/var/lib/rssify/rssify.db

