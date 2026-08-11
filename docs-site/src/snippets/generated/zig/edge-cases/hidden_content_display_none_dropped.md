---
id: fixture_zig_hidden_content_display_none_dropped
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
    const _result_json = try html_to_markdown.convert("<p>visible</p><div style=\"display:none\">secret hidden text</div><p>also visible</p>", null);
}

```
