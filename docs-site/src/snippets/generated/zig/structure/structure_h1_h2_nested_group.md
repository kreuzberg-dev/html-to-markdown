```zig title="Zig"
const std = @import("std");
const html_to_markdown = @import("html_to_markdown");

pub fn main() !void {
    const _result_json = try html_to_markdown.convert("<h1>Chapter One</h1><p>Chapter intro.</p><h2>Section One</h2><p>Section content.</p>", "{\"include_document_structure\":true}");
}

```
