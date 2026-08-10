```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>See the chart below:</p><figure><img src=\"chart.svg\"><figcaption>Revenue Trends 2020-2024</figcaption></figure><p>As shown in the chart above.</p>", new ConversionOptions());

```
