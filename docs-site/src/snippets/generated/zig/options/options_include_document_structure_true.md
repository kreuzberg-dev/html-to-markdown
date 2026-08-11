---
id: fixture_zig_options_include_document_structure_true
language: zig
target: zig
level: typecheck
requires: []
side_effect: safe
---

```zig title="Zig"
const std = @import("std");
const html_to_markdown = @import("html_to_markdown");

pub fn main() !void {
    const _result_json = try html_to_markdown.convert("<article><h1>Heading</h1><p>Paragraph body.</p></article>", "{\"include_document_structure\":true}");
}

```
