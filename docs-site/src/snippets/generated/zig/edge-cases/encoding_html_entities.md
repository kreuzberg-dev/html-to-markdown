---
id: fixture_zig_encoding_html_entities
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
    const _result_json = try html_to_markdown.convert("<p>&amp; &lt; &gt; &nbsp; &quot; &apos;</p>", null);
}

```
