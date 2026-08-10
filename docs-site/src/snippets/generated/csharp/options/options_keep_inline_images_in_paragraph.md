```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>Text <img src='icon.png' alt='icon'> more text</p>", new ConversionOptions { KeepInlineImagesIn = new List<string> { "p" } });

```
