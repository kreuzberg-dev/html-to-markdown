```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p><a href=\"https://example.com\" onclick=\"alert('xss')\">Click me</a></p><button onmouseover=\"steal_data()\">Hover me</button>", new ConversionOptions());

```
