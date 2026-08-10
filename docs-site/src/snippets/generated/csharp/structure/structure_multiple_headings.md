```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<h1>Main Title</h1><h2>Section One</h2><p>Section one content.</p><h2>Section Two</h2><p>Section two content.</p>", new ConversionOptions { IncludeDocumentStructure = true });

```
