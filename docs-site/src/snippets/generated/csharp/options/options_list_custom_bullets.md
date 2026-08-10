```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<ul><li>Item A</li><li>Item B</li></ul>", new ConversionOptions { Bullets = "*" });

```
