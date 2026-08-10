```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<html><head><title>Page</title><meta name=\"keywords\" content=\"rust, markdown, html, converter\"></head><body><p>Content</p></body></html>", new ConversionOptions { ExtractMetadata = true });

```
