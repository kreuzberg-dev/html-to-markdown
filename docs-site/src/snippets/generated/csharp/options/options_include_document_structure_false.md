---
id: fixture_csharp_options_include_document_structure_false
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<article><h1>Heading</h1><p>Paragraph body.</p></article>", new ConversionOptions { IncludeDocumentStructure = false });

```
