```zig title="Zig"
const std = @import("std");
const html_to_markdown = @import("html_to_markdown");

pub fn main() !void {
    const _result_json = try html_to_markdown.convert("<a href=\"https://example.com\"><img src=\"logo.png\" alt=\"Logo\"></a>", null);
}

```
