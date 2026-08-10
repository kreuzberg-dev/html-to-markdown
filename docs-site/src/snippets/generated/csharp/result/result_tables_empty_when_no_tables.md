```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>No tables here</p>", new ConversionOptions { IncludeDocumentStructure = true });

```
