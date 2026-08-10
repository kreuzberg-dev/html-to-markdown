```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<form><label>Username: <input type=\"text\" name=\"username\" value=\"\"></label><label>Password: <input type=\"password\" name=\"password\"></label></form>", new ConversionOptions());

```
