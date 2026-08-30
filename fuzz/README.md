# Fuzzing

Panic, hang, and output-growth fuzzing for `html_to_markdown_rs::convert`.

This directory is a standalone cargo package, excluded from the workspace: it needs
nightly and libFuzzer, and must not be pulled into `cargo test` or
`cargo clippy --all-targets` at the workspace root.

## Run

```bash
cargo install cargo-fuzz
cargo +nightly fuzz run convert -- -max_len=65536
```

## Seed the corpus first

Fuzzing raw bytes into an HTML parser mostly produces input the tokenizer discards in its
first few states. Seeding from real documents is what gets coverage past that:

```bash
mkdir -p fuzz/corpus/convert
find tools/benchmark-harness/fixtures -name '*.html' -exec cp {} fuzz/corpus/convert/ \;
```

## Scope

The oracle is only "must not crash". Correctness of the Markdown is covered elsewhere:

- `crates/html-to-markdown/tests/corpus_robustness.rs` — the same oracle, deterministic and
  CI-runnable, over the fixture corpus plus a seeded generator.
- `crates/html-to-markdown/tests/tier_parity_corpus.rs` — Tier-1 against Tier-2.
- `crates/html-to-markdown/tests/roundtrip_fixpoint.rs` — convert/render/convert stability.

A crash found here should be minimised with `cargo +nightly fuzz tmin` and then landed as an
ordinary regression test next to those, not left as a corpus artifact.
