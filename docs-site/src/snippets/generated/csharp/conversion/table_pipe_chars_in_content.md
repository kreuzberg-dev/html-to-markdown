```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<table><thead><tr><th>Expression</th><th>Result</th></tr></thead><tbody><tr><td>a | b</td><td>true</td></tr></tbody></table>", new ConversionOptions());

```
