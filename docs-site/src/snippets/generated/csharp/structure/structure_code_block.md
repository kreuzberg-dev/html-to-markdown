```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>Example code:</p><pre><code class=\"language-rust\">fn main() { println!(\"Hello\"); }</code></pre>", new ConversionOptions { IncludeDocumentStructure = true });

```
