```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>Hello world</p>", new ConversionOptions { ExcludeSelectors = new List<string> {  } });

```
