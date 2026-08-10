```csharp title="C#"
using System.Text.Json;
using HtmlToMarkdown;

var ConfigOptions = new JsonSerializerOptions { PropertyNameCaseInsensitive = true };
var result = HtmlToMarkdownConverter.Convert("<p>Foo</p><pre><code>1\n2\n</code></pre><p>Bar</p>", new ConversionOptions { CodeBlockStyle = JsonSerializer.Deserialize<CodeBlockStyle>("\"Backticks\"", ConfigOptions)! });

```
