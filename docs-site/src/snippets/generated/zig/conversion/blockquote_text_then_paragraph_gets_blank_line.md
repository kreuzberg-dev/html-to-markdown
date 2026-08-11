---
id: fixture_zig_blockquote_text_then_paragraph_gets_blank_line
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
    const _result_json = try html_to_markdown.convert("<blockquote>Just text, then <p>a paragraph</p></blockquote>", null);
}

```
