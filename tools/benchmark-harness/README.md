# Benchmark harness

`htmbench run` captures nine independently timed samples per fixture and records their median and median absolute
deviation (MAD). It also retains the best of the first three batches solely for method-compatible comparisons with the
temporary schema-v1 baseline. Result schema v2 records the OS, architecture, CPU model/count, full Rust compiler
identity, Cargo version, compile-time profile and flags, compiled core features, measurement mode/settings, forced tier,
visitor mode, iteration override, and runner image/class
when available. Hostname is diagnostic only and is never part of the compatibility key.

Run a normal capture and comparison with:

```sh
task bench:run
task bench:compare
```

The checked-in schema-v1 baseline remains a temporary percentage-only compatibility bridge. Comparisons print a
warning while that bridge is active. Once an approved calibration promotes the baseline and guardrails to schema v2,
comparison becomes strict: current and baseline provenance must exactly match the approved calibration provenance,
and every fixture must have a measured floor. A mismatch or missing floor is a configuration error, not a skip.

## Calibration and baseline promotion

A calibration campaign requires exactly 40 schema-v2, full-corpus captures from one commit on one quiet, pinned
runner. Name captures in acquisition order (`0001.json` through `0040.json`) because adjacent captures form the 20
pairs used by calibration. Then run:

```sh
RUNS_DIR=/path/to/campaign task bench:calibrate
```

Calibration accepts only `0001.json` through `0040.json` with strictly increasing RFC3339 capture timestamps. It
validates the full commit SHA, fixture inventory, input/output sizes, nine-sample records, OS, architecture, CPU
model/count, Rust compiler version/host, Cargo version, release profile, core features, and runner image/class before
writing either file. The promoted baseline uses dedicated calibrated records rather than pretending the 40 run medians
are one run's samples, and a shared campaign ID binds that baseline to its guardrails. For each fixture it stores the
median and MAD across the 40 run medians. The floor is the
nearest-rank p95 of the 20 absolute adjacent-pair deltas. Comparison fails only when the positive delta exceeds the
larger of the unchanged group-policy percentage (5%, 8%, 10%, or 30%) and that fixture floor.

Both output files are staged and backed up before promotion; if either promotion fails, both originals are restored.
Baseline promotion requires retained raw artifacts, comparable hardware, a quiet-runner record, and reviewer approval.
Never promote a baseline or populate floors merely to make CI green.

## Quiet-runner gate

Capture calibration or A/B evidence only when the runner is pinned, on external power, thermally stable, and has no
competing compiler, test, indexing, or benchmark workload. The five-minute load average must stay below 25% of the
logical CPU count before the first capture and throughout the campaign. Abort and retain no floors if any condition
fails.

Issue #461 A/B evidence belongs under `evidence/issue-461/<campaign>/`. Build both revisions first with separate target
directories and identical toolchain/features, then collect at least 20 full-corpus rounds in A/B/B/A order. Retain every
raw result and report per-fixture median/MAD, paired ratios, bootstrap confidence intervals, and aggregate geometric
mean. The evidence README defines the on-disk layout.
