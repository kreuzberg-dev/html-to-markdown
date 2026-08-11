---
id: fixture_csharp_hidden_content_display_none_dropped
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>visible</p><div style=\"display:none\">secret hidden text</div><p>also visible</p>", new ConversionOptions());

```
