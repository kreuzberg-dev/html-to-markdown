# Issue 461 evidence layout

Each campaign uses a unique directory and keeps capture order explicit:

```text
<campaign>/
  manifest.json
  quiet-gate.txt
  raw/
    round-01-a1.json
    round-01-b1.json
    round-01-b2.json
    round-01-a2.json
    ...
    round-20-a2.json
  report.json
  report.md
```

`manifest.json` records both full commit SHAs, separate target directories, toolchain, features, build commands, runner
identity, and capture timestamps. `quiet-gate.txt` retains the preflight and during-run load/thermal observations.
`report.json` is machine-readable analysis; `report.md` explains whether the data confirms code regression or runner
drift. Do not add synthetic, partial, or noisy captures to this directory.
