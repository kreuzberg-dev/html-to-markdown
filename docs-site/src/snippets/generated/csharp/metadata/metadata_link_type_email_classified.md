```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>Contact <a href=\"mailto:hello@example.com\">us</a> directly.</p>", new ConversionOptions { ExtractMetadata = true });

```
