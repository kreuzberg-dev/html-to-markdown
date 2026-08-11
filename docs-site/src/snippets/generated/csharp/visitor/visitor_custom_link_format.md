---
id: fixture_csharp_visitor_custom_link_format
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>Visit <a href=\"https://example.com\">Example</a> for more info.</p>", new ConversionOptions());

```
