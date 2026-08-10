```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<nav>NavSection</nav><p>Paragraph</p>", new ConversionOptions { Preprocessing = new PreprocessingOptions { Enabled = false } });

```
