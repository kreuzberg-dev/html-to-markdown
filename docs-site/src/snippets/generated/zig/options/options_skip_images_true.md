---
id: fixture_zig_options_skip_images_true
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
    const _result_json = try html_to_markdown.convert("<p>Before <img src='test.jpg' alt='photo'> After</p>", "{\"skip_images\":true}");
}

```
