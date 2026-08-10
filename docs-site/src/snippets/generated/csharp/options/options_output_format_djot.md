```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>Simple paragraph.</p>", new ConversionOptions { OutputFormat = OutputFormat.Djot });

```
