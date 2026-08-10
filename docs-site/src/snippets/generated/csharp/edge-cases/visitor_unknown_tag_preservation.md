```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<article><p>Article text</p><x-custom>Custom element with content</x-custom><p>More article text</p></article>", new ConversionOptions());

```
