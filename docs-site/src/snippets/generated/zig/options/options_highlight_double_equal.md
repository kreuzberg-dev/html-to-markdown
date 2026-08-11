---
id: fixture_zig_options_highlight_double_equal
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
    const _result_json = try html_to_markdown.convert("<p>Text with <mark>highlighted</mark> here.</p>", "{\"highlight_style\":\"DoubleEqual\"}");
}

```
