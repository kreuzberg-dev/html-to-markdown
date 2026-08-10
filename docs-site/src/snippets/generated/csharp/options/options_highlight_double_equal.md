```csharp title="C#"
using System.Text.Json;
using HtmlToMarkdown;

var ConfigOptions = new JsonSerializerOptions { PropertyNameCaseInsensitive = true };
var result = HtmlToMarkdownConverter.Convert("<p>Text with <mark>highlighted</mark> here.</p>", new ConversionOptions { HighlightStyle = JsonSerializer.Deserialize<HighlightStyle>("\"DoubleEqual\"", ConfigOptions)! });

```
