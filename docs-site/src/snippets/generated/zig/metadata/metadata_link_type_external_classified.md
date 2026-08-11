---
id: fixture_zig_metadata_link_type_external_classified
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
    const _result_json = try html_to_markdown.convert("<p>See <a href=\"https://example.com\">Example</a> for details.</p>", "{\"extract_metadata\":true}");
}

```
