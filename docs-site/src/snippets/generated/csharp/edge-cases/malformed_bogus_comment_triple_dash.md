```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<h1>One</h1>\n<!-- /// --->\n<p>Two</p>", new ConversionOptions());

```
