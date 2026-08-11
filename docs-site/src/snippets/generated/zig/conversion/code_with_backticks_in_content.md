---
id: fixture_zig_code_with_backticks_in_content
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
    const _result_json = try html_to_markdown.convert("<p>Use <code>`backtick` here</code> carefully.</p>", null);
}

```
