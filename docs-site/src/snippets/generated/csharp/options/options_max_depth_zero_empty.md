```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>Hello</p>", new ConversionOptions { MaxDepth = 0 });

```
