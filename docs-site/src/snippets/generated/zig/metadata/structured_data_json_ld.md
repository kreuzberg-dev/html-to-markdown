```zig title="Zig"
const std = @import("std");
const html_to_markdown = @import("html_to_markdown");

pub fn main() !void {
    const _result_json = try html_to_markdown.convert("<html><head><title>Article</title><script type=\"application/ld+json\">{\"@context\":\"https://schema.org\",\"@type\":\"Article\",\"headline\":\"My Article\",\"author\":{\"@type\":\"Person\",\"name\":\"Jane Doe\"},\"datePublished\":\"2024-01-15\"}</script></head><body><h1>My Article</h1><p>Article body text.</p></body></html>", null);
}

```
