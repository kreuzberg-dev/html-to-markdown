```csharp title="C#"
using System.Text.Json;
using HtmlToMarkdown;

var ConfigOptions = new JsonSerializerOptions { PropertyNameCaseInsensitive = true };
var result = HtmlToMarkdownConverter.Convert("<pre><code>let x = 1;</code></pre>", new ConversionOptions { CodeBlockStyle = JsonSerializer.Deserialize<CodeBlockStyle>("\"Tildes\"", ConfigOptions)! });

```
