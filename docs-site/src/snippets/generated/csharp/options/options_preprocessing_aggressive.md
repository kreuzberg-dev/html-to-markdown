```csharp title="C#"
using System.Text.Json;
using HtmlToMarkdown;

var ConfigOptions = new JsonSerializerOptions { PropertyNameCaseInsensitive = true };
var result = HtmlToMarkdownConverter.Convert("<nav>Menu</nav><article><h1>Title</h1><p>Content</p></article><aside>Sidebar</aside><footer>Footer</footer>", new ConversionOptions { Preprocessing = new PreprocessingOptions { Preset = JsonSerializer.Deserialize<PreprocessingPreset>("\"Aggressive\"", ConfigOptions)! } });

```
