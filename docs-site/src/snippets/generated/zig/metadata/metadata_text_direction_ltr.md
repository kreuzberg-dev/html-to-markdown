```zig title="Zig"
const std = @import("std");
const html_to_markdown = @import("html_to_markdown");

pub fn main() !void {
    const _result_json = try html_to_markdown.convert("<html lang=\"en\" dir=\"ltr\"><head><title>LTR Document</title></head><body><p>This is left-to-right text.</p></body></html>", "{\"extract_metadata\":true}");
}

```
