```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<pre><code>def hello(): pass</code></pre>", new ConversionOptions { CodeLanguage = "python" });

```
