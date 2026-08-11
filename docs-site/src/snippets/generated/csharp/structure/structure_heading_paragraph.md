---
id: fixture_csharp_structure_heading_paragraph
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<h1>Title</h1><p>A paragraph of text.</p>", new ConversionOptions { IncludeDocumentStructure = true });

```
