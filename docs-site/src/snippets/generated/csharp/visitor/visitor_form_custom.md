```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<div><form action=\"/submit\" method=\"POST\"><label>Name: <input type=\"text\" name=\"name\"></label><button type=\"submit\">Submit</button></form></div>", new ConversionOptions());

```
