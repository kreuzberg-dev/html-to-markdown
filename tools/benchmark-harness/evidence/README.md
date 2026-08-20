# Benchmark evidence

Raw calibration and A/B artifacts live under an issue/campaign directory and must never be synthesized from an
ordinary development run.

## Schema-v1 output metadata normalization

The exact-inventory policy compares fixture group, input bytes, and output bytes before evaluating timing. The #460
table-whitespace correction deterministically changed two fixture outputs, so the temporary schema-v1 baseline metadata
was normalized without changing any timing or threshold:

| Fixture | Previous bytes | Corrected bytes |
| --- | ---: | ---: |
| `real-world/issues/gh-190/ozonekorea.html` | 25541 | 25383 |
| `real-world/issues/gh-190/kimbrain.html` | 40082 | 40078 |

Review the baseline diff with zero context to verify these are its only changes:

```sh
git diff --unified=0 -- tools/benchmark-harness/baselines/baseline.json
```
