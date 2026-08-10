```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<form><label>Color:</label><select><option value=\"red\">Red</option><option value=\"blue\" selected>Blue</option><option value=\"green\">Green</option></select></form>", new ConversionOptions { Preprocessing = new PreprocessingOptions { RemoveForms = false } });

```
