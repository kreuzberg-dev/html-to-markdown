```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<body><div id=\"ad-container\">Buy stuff</div><p>Article text</p></body>", new ConversionOptions { ExcludeSelectors = new List<string> { "#ad-container" } });

```
