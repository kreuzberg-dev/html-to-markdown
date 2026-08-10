```zig title="Zig"
const std = @import("std");
const html_to_markdown = @import("html_to_markdown");

pub fn main() !void {
    const _result_json = try html_to_markdown.convert("<html><head><title>Article</title></head><body><article itemscope itemtype=\"https://schema.org/Article\"><h1 itemprop=\"headline\">Breaking News Today</h1><span itemprop=\"author\">Jane Reporter</span><span itemprop=\"datePublished\">2024-04-22</span><div itemprop=\"articleBody\"><p>The article content goes here with important information about the breaking news story.</p></div></article></body></html>", "{\"extract_metadata\":true}");
}

```
