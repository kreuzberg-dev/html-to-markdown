```csharp title="C#"
using System.Text.Json;
using HtmlToMarkdown;

var ConfigOptions = new JsonSerializerOptions { PropertyNameCaseInsensitive = true };
var result = HtmlToMarkdownConverter.Convert("<p>First<br>Second</p>", new ConversionOptions { NewlineStyle = JsonSerializer.Deserialize<NewlineStyle>("\"Spaces\"", ConfigOptions)! });

```
