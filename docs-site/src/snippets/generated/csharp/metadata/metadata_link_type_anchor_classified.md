```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>Jump to <a href=\"#section\">section</a> below.</p>", new ConversionOptions { ExtractMetadata = true });

```
