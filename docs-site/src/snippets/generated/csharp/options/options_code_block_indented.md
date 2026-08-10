```csharp title="C#"
using System.Text.Json;
using HtmlToMarkdown;

var ConfigOptions = new JsonSerializerOptions { PropertyNameCaseInsensitive = true };
var result = HtmlToMarkdownConverter.Convert("<pre><code>print('hello')</code></pre>", new ConversionOptions { CodeBlockStyle = JsonSerializer.Deserialize<CodeBlockStyle>("\"Indented\"", ConfigOptions)! });

```
