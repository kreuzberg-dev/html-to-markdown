```csharp
using HtmlToMarkdown;

// Binary data (detected via magic bytes) is rejected before parsing.
var html = "%PDF-1.4 not actually HTML";

try
{
    var result = HtmlToMarkdownConverter.Convert(html, null);
    Console.WriteLine(result.Content);
}
catch (InvalidInputException e)
{
    Console.Error.WriteLine($"invalid input: {e.Message}");
}
catch (ConversionErrorException e)
{
    Console.Error.WriteLine($"conversion failed: {e.Message}");
}
```
