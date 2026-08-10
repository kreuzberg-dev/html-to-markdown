```zig title="Zig"
const std = @import("std");
const html_to_markdown = @import("html_to_markdown");

pub fn main() !void {
    const _result_json = try html_to_markdown.convert("<html><head><meta name=\"twitter:card\" content=\"summary_large_image\"><meta name=\"twitter:site\" content=\"@examplesite\"><meta name=\"twitter:title\" content=\"Twitter Card Title\"><meta name=\"twitter:description\" content=\"Twitter card description.\"><meta name=\"twitter:image\" content=\"https://example.com/twitter-image.jpg\"></head><body><p>Content</p></body></html>", null);
}

```
