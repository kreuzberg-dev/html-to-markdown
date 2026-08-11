---
id: fixture_zig_options_br_in_tables_false
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
    const _result_json = try html_to_markdown.convert("<table><tr><th>Col</th></tr><tr><td>A<br>B</td></tr></table>", "{\"br_in_tables\":false}");
}

```
