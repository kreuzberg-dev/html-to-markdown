```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>The variable_name is defined.</p>", new ConversionOptions { EscapeUnderscores = true });

```
