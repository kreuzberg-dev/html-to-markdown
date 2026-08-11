---
id: fixture_zig_structure_heading_paragraph
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
    const _result_json = try html_to_markdown.convert("<h1>Title</h1><p>A paragraph of text.</p>", "{\"include_document_structure\":true}");
}

```
