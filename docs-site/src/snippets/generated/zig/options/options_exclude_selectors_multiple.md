```zig title="Zig"
const std = @import("std");
const html_to_markdown = @import("html_to_markdown");

pub fn main() !void {
    const _result_json = try html_to_markdown.convert("<body><nav class=\"nav\">Menu</nav><p>Content</p><footer>Footer</footer></body>", "{\"exclude_selectors\":[\".nav\",\"footer\"]}");
}

```
