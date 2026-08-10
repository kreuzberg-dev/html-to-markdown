```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>Glossary:</p><dl><dt>Term A</dt><dd>Definition of term A</dd><dt>Term B</dt><dd>Definition of term B</dd></dl><p>End of glossary</p>", new ConversionOptions());

```
