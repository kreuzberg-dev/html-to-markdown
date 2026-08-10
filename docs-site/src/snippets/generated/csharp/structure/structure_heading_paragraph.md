```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<h1>Title</h1><p>A paragraph of text.</p>", new ConversionOptions { IncludeDocumentStructure = true });

```
