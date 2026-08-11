---
id: fixture_zig_hidden_content_visibility_hidden_dropped
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
    const _result_json = try html_to_markdown.convert("<p>visible</p><span style=\"visibility:hidden\">secret hidden span</span><p>also visible</p>", null);
}

```
