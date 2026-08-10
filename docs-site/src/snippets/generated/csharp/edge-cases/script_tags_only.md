```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<html><head><script>alert('xss')</script></head><body><script>document.write('hello')</script></body></html>", new ConversionOptions());

```
