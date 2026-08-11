# Safety & Sanitization Domain

## Purpose

Input-integrity and runtime-safety checks on the conversion pipeline. This domain does **not** sanitize
output: converted Markdown is not safe to render as trusted content, and callers must sanitize it
downstream. See `SECURITY.md`.

## Key Areas

- **Input validation** (`validation.rs`, `validate_input()`): rejects binary/corrupted input — gzip/zstd
  magic bytes, zip/PDF magic bytes, excess NUL bytes, excess control-character ratio, undeclared
  UTF-16 (BOM or heuristic). This is a data-integrity check, not a content sanitizer — it does not
  inspect or remove HTML elements, attributes, or URLs.
- **No input size cap.** There is no configurable or hard-coded maximum input length.
- **Depth limiting**: `ConversionOptions::max_depth` bounds recursive DOM traversal (`walk_node` in
  `converter/main.rs`) to prevent native stack overflow on pathologically deep or adversarial HTML.
  Defaults to a conservative native-stack-safe limit and clamps any caller-supplied value to an
  absolute ceiling; cannot be disabled.
- **`ConversionError::SanitizationError`** exists in `error.rs` and is wired into every binding's error
  mapping, but the core library never constructs it — there is no sanitization pass that can trigger it
  today.
- Script and style tags are always stripped from the DOM walk (a markup-fidelity behavior, not a
  security control) — this does not remove `javascript:`/`data:` URLs, inline event handler attributes,
  or other injectable content, which pass through unchanged into the Markdown output.

## Architecture

`validate_input()` runs once, ahead of parsing, and rejects non-text input outright
(`ConversionError::InvalidInput`). There is no subsequent sanitize step: `parse -> convert` runs directly
on the validated-as-text (not validated-as-safe) input, and `max_depth` is enforced during the walk. No
`SafetyConfig`, URL scheme allow-list, or attribute allow-list exists in the codebase.

## Dependencies

- Downstream: HTML Parsing domain (operates on binary-validated but not content-sanitized input),
  Conversion Algorithms domain
