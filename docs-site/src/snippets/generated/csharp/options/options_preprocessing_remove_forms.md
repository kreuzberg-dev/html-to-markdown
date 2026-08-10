```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>Before</p><form><input type='text'/><button>Submit</button></form><p>After</p>", new ConversionOptions { Preprocessing = new PreprocessingOptions { RemoveForms = true } });

```
