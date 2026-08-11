---
id: fixture_csharp_hidden_content_visibility_hidden_dropped
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>visible</p><span style=\"visibility:hidden\">secret hidden span</span><p>also visible</p>", new ConversionOptions());

```
