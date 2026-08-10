```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>Main content here.</p><details><summary>Hidden section</summary><p>Secret details</p></details><p>More main content.</p>", new ConversionOptions());

```
