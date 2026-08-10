```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<table><thead><tr><th align=\"left\">Left</th><th align=\"center\">Center</th><th align=\"right\">Right</th></tr></thead><tbody><tr><td>L</td><td>C</td><td>R</td></tr></tbody></table>", new ConversionOptions());

```
