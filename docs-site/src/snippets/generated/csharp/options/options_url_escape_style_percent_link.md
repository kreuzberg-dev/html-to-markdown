```csharp title="C#"
using System.Text.Json;
using HtmlToMarkdown;

var ConfigOptions = new JsonSerializerOptions { PropertyNameCaseInsensitive = true };
var result = HtmlToMarkdownConverter.Convert("<a href=\"/file (1).pdf\">file</a>", new ConversionOptions { UrlEscapeStyle = JsonSerializer.Deserialize<UrlEscapeStyle>("\"percent\"", ConfigOptions)! });

```
