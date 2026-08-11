---
id: fixture_zig_script_tags_only
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
    const _result_json = try html_to_markdown.convert("<html><head><script>alert('xss')</script></head><body><script>document.write('hello')</script></body></html>", null);
}

```
