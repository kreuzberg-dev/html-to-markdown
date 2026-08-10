```csharp title="C#"
using System.Text.Json;
using HtmlToMarkdown;

var ConfigOptions = new JsonSerializerOptions { PropertyNameCaseInsensitive = true };
var result = HtmlToMarkdownConverter.Convert("<h1>Title</h1><h2>Subtitle</h2>", new ConversionOptions { HeadingStyle = JsonSerializer.Deserialize<HeadingStyle>("\"Atx\"", ConfigOptions)! });

```
