---
id: fixture_zig_options_wrap_enabled
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
    const _result_json = try html_to_markdown.convert("<p>This is a long paragraph that should be wrapped at the specified column width when the wrap option is enabled.</p>", "{\"wrap\":true,\"wrap_width\":40}");
}

```
