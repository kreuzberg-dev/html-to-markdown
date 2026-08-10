```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<html><head><title>Test Page</title><meta name='description' content='A test page'></head><body><p>Content</p></body></html>", new ConversionOptions { ExtractMetadata = true });

```
