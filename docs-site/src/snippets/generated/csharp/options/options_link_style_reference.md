```csharp title="C#"
using System.Text.Json;
using HtmlToMarkdown;

var ConfigOptions = new JsonSerializerOptions { PropertyNameCaseInsensitive = true };
var result = HtmlToMarkdownConverter.Convert("<p><a href='https://example.com'>Example</a> and <a href='https://other.com'>Other</a></p>", new ConversionOptions { LinkStyle = JsonSerializer.Deserialize<LinkStyle>("\"Reference\"", ConfigOptions)! });

```
