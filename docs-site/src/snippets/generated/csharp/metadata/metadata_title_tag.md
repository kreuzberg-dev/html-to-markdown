```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<html><head><title>My Page</title></head><body><p>Content</p></body></html>", new ConversionOptions { ExtractMetadata = true });

```
