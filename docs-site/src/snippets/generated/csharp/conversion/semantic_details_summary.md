```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<details><summary>Click to expand</summary><p>Hidden content here.</p></details>", new ConversionOptions());

```
