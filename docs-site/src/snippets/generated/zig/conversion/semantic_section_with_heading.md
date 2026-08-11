---
id: fixture_zig_semantic_section_with_heading
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
    const _result_json = try html_to_markdown.convert("<section><h3>Section Heading</h3><p>Section content.</p></section>", null);
}

```
