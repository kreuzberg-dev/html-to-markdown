---
id: fixture_zig_blockquote_code_block_indentation_preserved
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
    const _result_json = try html_to_markdown.convert("<blockquote><pre><code>line1\n    line2 indented</code></pre></blockquote>", null);
}

```
