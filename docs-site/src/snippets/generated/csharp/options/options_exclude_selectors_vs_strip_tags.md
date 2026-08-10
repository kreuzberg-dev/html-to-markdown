```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<body><div class=\"wrapper\"><p>Inner paragraph</p></div><p>Outer text</p></body>", new ConversionOptions { ExcludeSelectors = new List<string> { ".wrapper" } });

```
