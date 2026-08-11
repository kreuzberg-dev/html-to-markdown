---
id: fixture_zig_result_warnings_empty_for_clean_input
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
    const _result_json = try html_to_markdown.convert("<h1>Title</h1><p>Clean content with <a href='https://example.com'>a link</a>.</p>", null);
}

```
