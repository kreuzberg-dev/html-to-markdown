```zig title="Zig"
const std = @import("std");
const html_to_markdown = @import("html_to_markdown");

pub fn main() !void {
    const _result_json = try html_to_markdown.convert("<ul><li><p>First paragraph in item.</p><p>Second paragraph in item.</p></li><li>Simple item</li></ul>", null);
}

```
