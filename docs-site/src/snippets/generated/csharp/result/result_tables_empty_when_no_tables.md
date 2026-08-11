---
id: fixture_csharp_result_tables_empty_when_no_tables
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>No tables here</p>", new ConversionOptions { IncludeDocumentStructure = true });

```
