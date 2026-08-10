```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>Safe content.</p><script>alert('xss')</script><p>More safe content.</p>", new ConversionOptions());

```
