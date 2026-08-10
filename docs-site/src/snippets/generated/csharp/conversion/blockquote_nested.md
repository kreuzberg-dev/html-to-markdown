```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<blockquote><p>Outer quote.</p><blockquote><p>Inner quote.</p></blockquote></blockquote>", new ConversionOptions());

```
