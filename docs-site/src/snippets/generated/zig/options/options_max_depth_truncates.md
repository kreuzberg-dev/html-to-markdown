```zig title="Zig"
const std = @import("std");
const html_to_markdown = @import("html_to_markdown");

pub fn main() !void {
    const _result_json = try html_to_markdown.convert("<div><p>Shallow</p><div><div><div><p>Too deep</p></div></div></div></div>", "{\"max_depth\":3}");
}

```
