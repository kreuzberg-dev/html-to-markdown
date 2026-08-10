```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<body><div class=\"nav\">Navigation</div><p>Article body</p></body>", new ConversionOptions { ExcludeSelectors = new List<string> { ".nav" }, OutputFormat = OutputFormat.Plain });

```
