---
id: fixture_zig_hidden_content_aria_hidden_still_rendered
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
    const _result_json = try html_to_markdown.convert("<p>visible</p><div aria-hidden=\"true\">still shown</div><p>also visible</p>", null);
}

```
