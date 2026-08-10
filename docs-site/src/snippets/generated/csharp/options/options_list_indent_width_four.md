```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<ul><li>Outer<ul><li>Inner</li></ul></li></ul>", new ConversionOptions { ListIndentWidth = 4 });

```
