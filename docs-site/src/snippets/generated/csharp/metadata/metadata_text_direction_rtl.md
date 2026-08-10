```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<html lang=\"ar\" dir=\"rtl\"><head><title>RTL Document</title></head><body><p>This is right-to-left text.</p></body></html>", new ConversionOptions { ExtractMetadata = true });

```
