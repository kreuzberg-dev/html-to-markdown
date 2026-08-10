```zig title="Zig"
const std = @import("std");
const html_to_markdown = @import("html_to_markdown");

pub fn main() !void {
    const _result_json = try html_to_markdown.convert("<p>Items:</p><ul><li>Alpha</li><li>Beta</li><li>Gamma</li></ul>", "{\"include_document_structure\":true}");
}

```
