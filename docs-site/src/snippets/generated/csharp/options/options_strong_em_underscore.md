```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p><strong>bold</strong> and <em>italic</em></p>", new ConversionOptions { StrongEmSymbol = "_" });

```
