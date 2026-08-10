```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>Use 2*3 = 6 in math.</p>", new ConversionOptions { EscapeAsterisks = true });

```
