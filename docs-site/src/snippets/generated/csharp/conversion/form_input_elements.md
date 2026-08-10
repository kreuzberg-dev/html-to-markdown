```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<form><label for=\"name\">Name:</label><input type=\"text\" id=\"name\" placeholder=\"Enter name\"></form>", new ConversionOptions { Preprocessing = new PreprocessingOptions { RemoveForms = false } });

```
