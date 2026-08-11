---
id: fixture_zig_options_preprocessing_remove_forms
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
    const _result_json = try html_to_markdown.convert("<p>Before</p><form><input type='text'/><button>Submit</button></form><p>After</p>", "{\"preprocessing\":{\"remove_forms\":true}}");
}

```
