```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<html><head><title>Page</title><link rel=\"canonical\" href=\"https://example.com/canonical-page\"></head><body><p>Content</p></body></html>", new ConversionOptions { ExtractMetadata = true });

```
