```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<table><thead><tr><th colspan=\"2\">Full Name</th></tr></thead><tbody><tr><td>John</td><td>Doe</td></tr></tbody></table>", new ConversionOptions());

```
