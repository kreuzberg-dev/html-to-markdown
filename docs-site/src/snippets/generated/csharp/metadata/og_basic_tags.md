```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<html><head><title>Fallback Title</title><meta property=\"og:title\" content=\"OG Title\"><meta property=\"og:description\" content=\"OG description text.\"><meta property=\"og:image\" content=\"https://example.com/image.jpg\"></head><body><p>Content</p></body></html>", new ConversionOptions());

```
