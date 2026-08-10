```csharp title="C#"
using System.Text.Json;
using HtmlToMarkdown;

var ConfigOptions = new JsonSerializerOptions { PropertyNameCaseInsensitive = true };
var result = HtmlToMarkdownConverter.Convert("<h1>Main Title</h1>", new ConversionOptions { HeadingStyle = JsonSerializer.Deserialize<HeadingStyle>("\"Underlined\"", ConfigOptions)! });

```
