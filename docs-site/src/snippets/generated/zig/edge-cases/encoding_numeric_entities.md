---
id: fixture_zig_encoding_numeric_entities
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
    const _result_json = try html_to_markdown.convert("<p>Copyright: &#169; Trade: &#174; Euro: &#8364; Hex: &#x00A9;</p>", null);
}

```
