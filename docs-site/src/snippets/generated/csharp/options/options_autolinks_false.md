```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p><a href='https://example.com'>https://example.com</a></p>", new ConversionOptions { Autolinks = false });

```
