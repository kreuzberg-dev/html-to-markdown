```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<table><tr><th>Col</th></tr><tr><td>A<br>B</td></tr></table>", new ConversionOptions { BrInTables = false });

```
