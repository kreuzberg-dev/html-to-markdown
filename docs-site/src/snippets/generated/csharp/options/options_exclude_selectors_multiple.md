```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<body><nav class=\"nav\">Menu</nav><p>Content</p><footer>Footer</footer></body>", new ConversionOptions { ExcludeSelectors = new List<string> { ".nav", "footer" } });

```
