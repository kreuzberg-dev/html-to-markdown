---
id: fixture_csharp_hidden_content_aria_hidden_still_rendered
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>visible</p><div aria-hidden=\"true\">still shown</div><p>also visible</p>", new ConversionOptions());

```
