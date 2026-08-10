```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>&amp; &lt; &gt; &nbsp; &quot; &apos;</p>", new ConversionOptions());

```
