---
id: fixture_zig_issue_396_backticks_blank_line_after_fence
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
    const _result_json = try html_to_markdown.convert("<p>Foo</p><pre><code>1\n2\n</code></pre><p>Bar</p>", "{\"code_block_style\":\"Backticks\"}");
}

```
