```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>Unclosed paragraph<div>Mixed nesting</p></div>", new ConversionOptions());

```
