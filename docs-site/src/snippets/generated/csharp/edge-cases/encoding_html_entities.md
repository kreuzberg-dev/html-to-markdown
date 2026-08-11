---
id: fixture_csharp_encoding_html_entities
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>&amp; &lt; &gt; &nbsp; &quot; &apos;</p>", new ConversionOptions());

```
