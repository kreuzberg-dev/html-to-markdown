```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<h1>Title</h1><p>Some <strong>bold</strong> text.</p>", new ConversionOptions { OutputFormat = OutputFormat.Plain });

```
