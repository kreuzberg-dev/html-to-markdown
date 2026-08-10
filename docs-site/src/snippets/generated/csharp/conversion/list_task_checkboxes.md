```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<ul><li><input type=\"checkbox\" checked> Done task</li><li><input type=\"checkbox\"> Pending task</li></ul>", new ConversionOptions());

```
