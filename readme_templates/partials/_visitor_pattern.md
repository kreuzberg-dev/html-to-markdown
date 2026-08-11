The visitor pattern enables custom HTML→Markdown conversion logic by providing callbacks for specific HTML elements during traversal. Pass a visitor as the third argument to `convert()`.

**Use Cases:**

- **Custom Markdown dialects** – Convert to Obsidian, Notion, or other flavors
- **Content filtering** – Remove tracking pixels, ads, or unwanted elements
- **URL rewriting** – Rewrite CDN URLs, add query parameters, validate links
- **Accessibility validation** – Check alt text, heading hierarchy, link text
- **Analytics** – Track element usage, link destinations, image sources

**Supported Visitor Methods:** 40+ callbacks for text, inline elements, links, images, headings, lists, blocks, and tables.

### Example: Quick Start

{% if language == 'python' %}

```python
from html_to_markdown import convert

class MyVisitor:
    def visit_link(self, ctx, href, text, title):
        # Rewrite CDN URLs
        if href.startswith("https://old-cdn.com"):
            href = href.replace("https://old-cdn.com", "https://new-cdn.com")
        return {"type": "custom", "output": f"[{text}]({href})"}

    def visit_image(self, ctx, src, alt, title):
        # Skip tracking pixels
        if "tracking" in src:
            return {"type": "skip"}
        return {"type": "continue"}

html = '<a href="https://old-cdn.com/file.pdf">Download</a>'
result = convert(html, visitor=MyVisitor())
markdown = result.content
```

{% elif language == 'typescript' %}

```typescript
import { convert, NodeContext, VisitResult } from "@xberg-io/html-to-markdown";

// `visitor` is a plain object of camelCase callbacks — there is no exported
// `Visitor` type. `VisitResult` is a type, not a runtime enum: return the
// lowercase string `"continue"` / `"skip"` / `"preserve_html"` for the
// built-in behaviors, or `{ custom: "..." }` to replace a node's output.
const visitor = {
  visitLink(ctx: NodeContext, href: string, text: string, title?: string) {
    // Rewrite CDN URLs
    if (href.startsWith("https://old-cdn.com")) {
      href = href.replace("https://old-cdn.com", "https://new-cdn.com");
    }
    return { custom: `[${text}](${href})` };
  },

  visitImage(ctx: NodeContext, src: string, alt?: string, title?: string): VisitResult {
    // Skip tracking pixels
    return src.includes("tracking") ? "skip" : "continue";
  },
};

const html = '<a href="https://old-cdn.com/file.pdf">Download</a>';
const result = convert(html, { visitor });
const markdown = result.content;
```

{% elif language == 'ruby' %}

```ruby
require 'html_to_markdown'

class MyVisitor
  def visit_link(ctx, href, text, title = nil)
    # Rewrite CDN URLs
    if href.start_with?('https://old-cdn.com')
      href = href.sub('https://old-cdn.com', 'https://new-cdn.com')
    end
    # Directive keys/symbols are matched case-sensitively as lowercase: :custom, :skip, :continue.
    { custom: "[#{text}](#{href})" }
  end

  def visit_image(ctx, src, alt = nil, title = nil)
    # Skip tracking pixels
    src.include?('tracking') ? :skip : :continue
  end
end

html = '<a href="https://old-cdn.com/file.pdf">Download</a>'
# The visitor is the second positional argument — it cannot be combined
# with an options Hash in the same call.
result = HtmlToMarkdown.convert(html, MyVisitor.new)
markdown = result.content
```

{% elif language == 'php' %}

```php
<?php
use HtmlToMarkdown\HtmlToMarkdownApi;
use HtmlToMarkdown\ConversionOptions;
use HtmlToMarkdown\VisitorHandle;

// Visitors are duck-typed: define any subset of visit_* methods.
$visitor = new class {
    public function visit_link($ctx, $href, $text, $title) {
        // Rewrite CDN URLs
        if (str_starts_with($href, 'https://old-cdn.com')) {
            $href = str_replace('https://old-cdn.com', 'https://new-cdn.com', $href);
        }
        return ['custom' => "[{$text}]({$href})"];
    }

    public function visit_image($ctx, $src, $alt, $title) {
        // Skip tracking pixels
        return str_contains($src, 'tracking') ? 'skip' : 'continue';
    }
};

$html = '<a href="https://old-cdn.com/file.pdf">Download</a>';
$visitorHandle = VisitorHandle::from_php_object($visitor);
$options = ConversionOptions::from_json('{}')->withVisitor($visitorHandle);
$result = HtmlToMarkdownApi::convert($html, $options);
$markdown = $result->content;
```

{% elif language == 'csharp' %}

```csharp
using HtmlToMarkdown;

var html = "<a href=\"https://old-cdn.com/file.pdf\">Download</a>";
var options = new ConversionOptions { Visitor = new MyVisitor() };
var result = {{ csharp_wrapper_class }}.Convert(html, options);
var markdown = result.Content;

public sealed class MyVisitor : IHtmlVisitor
{
    public VisitResult VisitLink(NodeContext ctx, string href, string text, string title)
    {
        // Rewrite CDN URLs
        if (href.StartsWith("https://old-cdn.com"))
        {
            href = href.Replace("https://old-cdn.com", "https://new-cdn.com");
        }
        return new VisitResult.Custom($"[{text}]({href})");
    }

    public VisitResult VisitImage(NodeContext ctx, string src, string alt, string title) =>
        // Skip tracking pixels
        src.Contains("tracking") ? new VisitResult.Skip() : new VisitResult.Continue();

    // All other callbacks default to Continue; IHtmlVisitor requires all 37 to be implemented.
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

{% elif language == 'elixir' %}

```elixir
visitor = %{
  handle_link: fn %{"href" => href, "text" => text} ->
    # Rewrite CDN URLs
    href =
      if String.starts_with?(href, "https://old-cdn.com") do
        String.replace(href, "https://old-cdn.com", "https://new-cdn.com")
      else
        href
      end

    {:custom, "[#{text}](#{href})"}
  end,
  handle_image: fn %{"src" => src} ->
    # Skip tracking pixels
    if String.contains?(src, "tracking"), do: :skip, else: :continue
  end
}

html = "<a href=\"https://old-cdn.com/file.pdf\">Download</a>"
{:ok, result} = HtmlToMarkdown.convert(html, %{visitor: visitor})
result.content
```

{% elif language == 'r' %}

```r
library(htmltomarkdown)

visitor <- list(
  visit_link = function(ctx, href, text, title) {
    # Rewrite CDN URLs
    if (startsWith(href, "https://old-cdn.com")) {
      href <- sub("https://old-cdn.com", "https://new-cdn.com", href, fixed = TRUE)
    }
    list(custom = paste0("[", text, "](", href, ")"))
  },
  visit_image = function(ctx, src, alt, title) {
    # Skip tracking pixels
    if (grepl("tracking", src, fixed = TRUE)) {
      "skip"
    } else {
      "continue"
    }
  }
)

html <- '<a href="https://old-cdn.com/file.pdf">Download</a>'
result <- convert(html, options = list(visitor = visitor))
cat(result$content)
```

{% endif %}
