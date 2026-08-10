```csharp title="C#"
using System.Text.Json;
using HtmlToMarkdown;

var ConfigOptions = new JsonSerializerOptions { PropertyNameCaseInsensitive = true };
var result = HtmlToMarkdownConverter.Convert("<pre><code>some code</code></pre>", new ConversionOptions { CodeBlockStyle = JsonSerializer.Deserialize<CodeBlockStyle>("\"Tildes\"", ConfigOptions)! });

```
