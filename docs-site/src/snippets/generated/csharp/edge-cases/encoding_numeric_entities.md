---
id: fixture_csharp_encoding_numeric_entities
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>Copyright: &#169; Trade: &#174; Euro: &#8364; Hex: &#x00A9;</p>", new ConversionOptions());

```
