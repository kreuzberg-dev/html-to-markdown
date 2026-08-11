---
id: fixture_zig_options_link_style_reference
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
    const _result_json = try html_to_markdown.convert("<p><a href='https://example.com'>Example</a> and <a href='https://other.com'>Other</a></p>", "{\"link_style\":\"Reference\"}");
}

```
