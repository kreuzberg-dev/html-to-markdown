---
id: fixture_csharp_hidden_content_noscript_element_dropped
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>visible</p><noscript><p>secret noscript text</p></noscript><p>also visible</p>", new ConversionOptions());

```
