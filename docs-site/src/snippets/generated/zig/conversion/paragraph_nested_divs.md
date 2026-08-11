---
id: fixture_zig_paragraph_nested_divs
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
    const _result_json = try html_to_markdown.convert("<div><div><p>Nested text</p></div></div>", null);
}

```
