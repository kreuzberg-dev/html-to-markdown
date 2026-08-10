```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<ul><li><p>First paragraph in item.</p><p>Second paragraph in item.</p></li><li>Simple item</li></ul>", new ConversionOptions());

```
