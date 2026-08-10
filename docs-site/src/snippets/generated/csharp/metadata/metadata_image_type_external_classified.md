```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p><img src=\"https://example.com/photo.jpg\" alt=\"A photo\"></p>", new ConversionOptions { ExtractMetadata = true });

```
