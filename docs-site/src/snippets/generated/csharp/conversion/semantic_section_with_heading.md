---
id: fixture_csharp_semantic_section_with_heading
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<section><h3>Section Heading</h3><p>Section content.</p></section>", new ConversionOptions());

```
