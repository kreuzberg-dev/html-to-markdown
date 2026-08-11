---
id: fixture_csharp_link_anchor_fragment
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<a href=\"#section\">Jump to section</a>", new ConversionOptions());

```
