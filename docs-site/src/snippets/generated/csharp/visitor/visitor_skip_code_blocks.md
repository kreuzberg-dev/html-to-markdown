```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>Intro text</p><pre><code>let x = 42;</code></pre><p>Outro text</p>", new ConversionOptions());

```
