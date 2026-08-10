```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>Items:</p><ul><li>Alpha</li><li>Beta</li><li>Gamma</li></ul>", new ConversionOptions { IncludeDocumentStructure = true });

```
