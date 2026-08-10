```csharp title="C#"
using System.Text.Json;
using HtmlToMarkdown;

var ConfigOptions = new JsonSerializerOptions { PropertyNameCaseInsensitive = true };
var result = HtmlToMarkdownConverter.Convert("<img src=\"/img (1) <draft>.png\" alt=\"alt\">", new ConversionOptions { UrlEscapeStyle = JsonSerializer.Deserialize<UrlEscapeStyle>("\"percent\"", ConfigOptions)! });

```
