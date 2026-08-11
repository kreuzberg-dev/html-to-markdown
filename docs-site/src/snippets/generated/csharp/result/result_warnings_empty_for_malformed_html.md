---
id: fixture_csharp_result_warnings_empty_for_malformed_html
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>Unclosed paragraph<div>Mixed nesting</p></div>", new ConversionOptions());

```
