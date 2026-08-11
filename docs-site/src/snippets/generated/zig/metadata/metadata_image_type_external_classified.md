---
id: fixture_zig_metadata_image_type_external_classified
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
    const _result_json = try html_to_markdown.convert("<p><img src=\"https://example.com/photo.jpg\" alt=\"A photo\"></p>", "{\"extract_metadata\":true}");
}

```
