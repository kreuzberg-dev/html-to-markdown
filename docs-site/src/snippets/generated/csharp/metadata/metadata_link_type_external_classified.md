```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>See <a href=\"https://example.com\">Example</a> for details.</p>", new ConversionOptions { ExtractMetadata = true });

```
