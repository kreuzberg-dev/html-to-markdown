```csharp title="C#"
using System.Text.Json;
using HtmlToMarkdown;

var ConfigOptions = new JsonSerializerOptions { PropertyNameCaseInsensitive = true };
var result = HtmlToMarkdownConverter.Convert("<p>Line one<br>Line two</p>", new ConversionOptions { NewlineStyle = JsonSerializer.Deserialize<NewlineStyle>("\"Backslash\"", ConfigOptions)! });

```
