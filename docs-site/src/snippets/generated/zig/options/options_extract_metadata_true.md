```zig title="Zig"
const std = @import("std");
const html_to_markdown = @import("html_to_markdown");

pub fn main() !void {
    const _result_json = try html_to_markdown.convert("<html><head><title>Test Page</title><meta name='description' content='A test page'></head><body><p>Content</p></body></html>", "{\"extract_metadata\":true}");
}

```
