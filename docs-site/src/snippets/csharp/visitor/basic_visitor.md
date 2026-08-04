```csharp
using HtmlToMarkdown;

var html = "<a href=\"https://old-cdn.com/file.pdf\">Download</a>";
var options = new ConversionOptions { Visitor = new MyVisitor() };
var result = HtmlToMarkdownConverter.Convert(html, options);
Console.WriteLine(result.Content);

public sealed class MyVisitor : IHtmlVisitor
{
    public VisitResult VisitLink(NodeContext ctx, string href, string text, string title)
    {
        // Rewrite CDN URLs.
        if (href.StartsWith("https://old-cdn.com"))
        {
            href = href.Replace("https://old-cdn.com", "https://new-cdn.com");
        }
        return new VisitResult.Custom($"[{text}]({href})");
    }

    public VisitResult VisitImage(NodeContext ctx, string src, string alt, string title) =>
        src.Contains("tracking") ? new VisitResult.Skip() : new VisitResult.Continue();

    public VisitResult VisitText(NodeContext ctx, string text) => new VisitResult.Continue();
    public VisitResult VisitElementStart(NodeContext ctx) => new VisitResult.Continue();
    public VisitResult VisitElementEnd(NodeContext ctx, string output) => new VisitResult.Continue();
    public VisitResult VisitHeading(NodeContext ctx, uint level, string text, string id) => new VisitResult.Continue();
    public VisitResult VisitCodeBlock(NodeContext ctx, string lang, string code) => new VisitResult.Continue();
    public VisitResult VisitCodeInline(NodeContext ctx, string code) => new VisitResult.Continue();
    public VisitResult VisitListItem(NodeContext ctx, bool ordered, string marker, string text) => new VisitResult.Continue();
    public VisitResult VisitListStart(NodeContext ctx, bool ordered) => new VisitResult.Continue();
    public VisitResult VisitListEnd(NodeContext ctx, bool ordered, string output) => new VisitResult.Continue();
    public VisitResult VisitTableStart(NodeContext ctx) => new VisitResult.Continue();
    public VisitResult VisitTableRow(NodeContext ctx, List<string> cells, bool isHeader) => new VisitResult.Continue();
    public VisitResult VisitTableEnd(NodeContext ctx, string output) => new VisitResult.Continue();
    public VisitResult VisitBlockquote(NodeContext ctx, string content, ulong depth) => new VisitResult.Continue();
    public VisitResult VisitStrong(NodeContext ctx, string text) => new VisitResult.Continue();
    public VisitResult VisitEmphasis(NodeContext ctx, string text) => new VisitResult.Continue();
    public VisitResult VisitStrikethrough(NodeContext ctx, string text) => new VisitResult.Continue();
    public VisitResult VisitUnderline(NodeContext ctx, string text) => new VisitResult.Continue();
    public VisitResult VisitSubscript(NodeContext ctx, string text) => new VisitResult.Continue();
    public VisitResult VisitSuperscript(NodeContext ctx, string text) => new VisitResult.Continue();
    public VisitResult VisitMark(NodeContext ctx, string text) => new VisitResult.Continue();
    public VisitResult VisitLineBreak(NodeContext ctx) => new VisitResult.Continue();
    public VisitResult VisitHorizontalRule(NodeContext ctx) => new VisitResult.Continue();
    public VisitResult VisitCustomElement(NodeContext ctx, string tagName, string html) => new VisitResult.Continue();
    public VisitResult VisitDefinitionListStart(NodeContext ctx) => new VisitResult.Continue();
    public VisitResult VisitDefinitionTerm(NodeContext ctx, string text) => new VisitResult.Continue();
    public VisitResult VisitDefinitionDescription(NodeContext ctx, string text) => new VisitResult.Continue();
    public VisitResult VisitDefinitionListEnd(NodeContext ctx, string output) => new VisitResult.Continue();
    public VisitResult VisitForm(NodeContext ctx, string action, string method) => new VisitResult.Continue();
    public VisitResult VisitInput(NodeContext ctx, string inputType, string name, string value) => new VisitResult.Continue();
    public VisitResult VisitButton(NodeContext ctx, string text) => new VisitResult.Continue();
    public VisitResult VisitAudio(NodeContext ctx, string src) => new VisitResult.Continue();
    public VisitResult VisitVideo(NodeContext ctx, string src) => new VisitResult.Continue();
    public VisitResult VisitIframe(NodeContext ctx, string src) => new VisitResult.Continue();
    public VisitResult VisitDetails(NodeContext ctx, bool open) => new VisitResult.Continue();
    public VisitResult VisitSummary(NodeContext ctx, string text) => new VisitResult.Continue();
    public VisitResult VisitFigureStart(NodeContext ctx) => new VisitResult.Continue();
    public VisitResult VisitFigcaption(NodeContext ctx, string text) => new VisitResult.Continue();
    public VisitResult VisitFigureEnd(NodeContext ctx, string output) => new VisitResult.Continue();
}
```
