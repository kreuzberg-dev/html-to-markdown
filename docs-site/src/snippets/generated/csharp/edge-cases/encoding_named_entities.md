```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>Em dash&mdash;used for parenthetical remarks&mdash;is common. Ellipsis&hellip; indicates omission. Non-breaking&nbsp;space.</p>", new ConversionOptions());

```
