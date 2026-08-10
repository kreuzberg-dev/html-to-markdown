```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<body><div class=\"cookie-banner\">Accept cookies</div><p>Main content</p></body>", new ConversionOptions { ExcludeSelectors = new List<string> { ".cookie-banner" } });

```
