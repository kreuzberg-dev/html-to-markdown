---
id: fixture_zig_result_tables_empty_when_no_tables
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
    const _result_json = try html_to_markdown.convert("<p>No tables here</p>", "{\"include_document_structure\":true}");
}

```
