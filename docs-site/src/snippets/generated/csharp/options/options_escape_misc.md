```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>Use # and | and ~ in text.</p>", new ConversionOptions { EscapeMisc = true });

```
