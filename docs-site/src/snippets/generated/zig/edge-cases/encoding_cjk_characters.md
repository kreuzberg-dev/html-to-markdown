```zig title="Zig"
const std = @import("std");
const html_to_markdown = @import("html_to_markdown");

pub fn main() !void {
    const _result_json = try html_to_markdown.convert("<p>中文内容</p><p>日本語テキスト</p><p>한국어 텍스트</p>", null);
}

```
