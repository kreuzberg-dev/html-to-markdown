```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<article><h1>Heading</h1><p>Paragraph body.</p></article>", new ConversionOptions { IncludeDocumentStructure = false });

```
