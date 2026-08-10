```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>Text<img src=\"data:BADMIME\" alt=\"broken\">end</p>", new ConversionOptions { ExtractImages = true });

```
