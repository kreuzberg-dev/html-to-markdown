```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>Before <img src='test.jpg' alt='photo'> After</p>", new ConversionOptions { SkipImages = true });

```
