```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>First paragraph.</p><p>Second paragraph.</p>", new ConversionOptions { StripNewlines = true });

```
