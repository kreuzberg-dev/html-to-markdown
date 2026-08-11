---
id: fixture_zig_metadata_extract_all_images
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
    const _result_json = try html_to_markdown.convert("<html><head><title>Gallery</title></head><body><img src=\"https://example.com/photo1.jpg\" alt=\"Photo 1\"><img src=\"https://example.com/photo2.png\" alt=\"Photo 2\"><img src=\"/local/image.webp\" alt=\"Local image\"></body></html>", "{\"extract_metadata\":true}");
}

```
