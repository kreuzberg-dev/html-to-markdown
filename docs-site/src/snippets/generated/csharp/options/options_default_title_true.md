```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p><a href='https://example.com'>Link</a></p>", new ConversionOptions { DefaultTitle = true });

```
