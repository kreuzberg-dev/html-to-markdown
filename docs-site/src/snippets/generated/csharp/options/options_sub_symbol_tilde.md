```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>H<sub>2</sub>O</p>", new ConversionOptions { SubSymbol = "~" });

```
