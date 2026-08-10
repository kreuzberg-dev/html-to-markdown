```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>Text with # hash and [brackets] and * star</p>", new ConversionOptions { EscapeAscii = true });

```
