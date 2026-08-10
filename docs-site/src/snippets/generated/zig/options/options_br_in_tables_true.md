```zig title="Zig"
const std = @import("std");
const html_to_markdown = @import("html_to_markdown");

pub fn main() !void {
    const _result_json = try html_to_markdown.convert("<table><tr><th>Header</th></tr><tr><td>Line 1<br>Line 2</td></tr></table>", "{\"br_in_tables\":true}");
}

```
