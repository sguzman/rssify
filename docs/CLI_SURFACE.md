# CLI Surface (Adapter, no business logic)

Binary name: rssify

## Subcommands
- fetch
  --from <path?>       Path to feeds.json
  --store <repo?>      Destination repo spec, e.g. "fs:/path"
  --json               Emit JSON to stdout
  -v / --verbose       Increase verbosity (additive)
- stats
  --store <repo?>      Repo to read from
  --json               Emit JSON to stdout
- import
  --file <path?>       Newline-delimited URLs
  --out <path?>        Output feeds.json
  --json               Emit JSON to stdout
- add <url>
  --out <path?>        Output feeds.json
  --json               Emit JSON to stdout

Rules:
- The CLI must not contain business logic.
- It parses args, constructs typed requests, and calls core traits (future steps).
- Keep arguments stable; changes require a migration note.

