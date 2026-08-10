```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<html><head><style>body { color: red; }</style></head><body><style>.foo { margin: 0; }</style></body></html>", new ConversionOptions());

```
