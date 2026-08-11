---
id: fixture_csharp_metadata_extract_all_links
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<html><head><title>Links Page</title></head><body><p>Visit <a href=\"https://example.com\">Example</a> or <a href=\"https://docs.example.com\">Docs</a>.</p><p>Also see <a href=\"/relative/path\">relative link</a> and <a href=\"mailto:hello@example.com\">email us</a>.</p></body></html>", new ConversionOptions { ExtractMetadata = true });

```
