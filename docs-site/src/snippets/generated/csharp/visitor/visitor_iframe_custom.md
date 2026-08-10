```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>Embedded map:</p><iframe src=\"https://maps.example.com/embed\" width=\"400\" height=\"300\"></iframe><p>End of map</p>", new ConversionOptions());

```
