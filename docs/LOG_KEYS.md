# Structured log keys (project-wide)

- component: stable component name (core.ids, core.sched, repo.fs, adapter.cli)
- op: short operation label (parse, fetch, persist, decide, import, stats)
- feed_id: opaque identifier for a feed (string form)
- elapsed_ms: integer duration of the op
- items: integer count of items touched/emitted

