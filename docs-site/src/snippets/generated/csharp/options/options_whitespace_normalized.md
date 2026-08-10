```csharp title="C#"
using System.Text.Json;
using HtmlToMarkdown;

var ConfigOptions = new JsonSerializerOptions { PropertyNameCaseInsensitive = true };
var result = HtmlToMarkdownConverter.Convert("<p>Text   with    extra   spaces.</p>", new ConversionOptions { WhitespaceMode = JsonSerializer.Deserialize<WhitespaceMode>("\"Normalized\"", ConfigOptions)! });

```
