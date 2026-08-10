```zig title="Zig"
const std = @import("std");
const html_to_markdown = @import("html_to_markdown");

pub fn main() !void {
    const _result_json = try html_to_markdown.convert("<p>Jump to <a href=\"#section\">section</a> below.</p>", "{\"extract_metadata\":true}");
}

```
