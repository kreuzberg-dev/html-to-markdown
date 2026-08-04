```csharp
using HtmlToMarkdown;

var options = new ConversionOptions
{
    HeadingStyle = HeadingStyle.Atx,
    Wrap = true,
    WrapWidth = 80,
    ListIndentWidth = 4,
};

var html = "<h1>Hello</h1><p>This is <strong>formatted</strong> content.</p>";
var result = HtmlToMarkdownConverter.Convert(html, options);
Console.WriteLine(result.Content);
```
