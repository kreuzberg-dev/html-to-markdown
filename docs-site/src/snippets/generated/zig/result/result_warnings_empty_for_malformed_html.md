---
id: fixture_zig_result_warnings_empty_for_malformed_html
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
    const _result_json = try html_to_markdown.convert("<p>Unclosed paragraph<div>Mixed nesting</p></div>", null);
}

```
