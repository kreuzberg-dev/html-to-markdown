---
id: fixture_csharp_code_with_backticks_in_content
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>Use <code>`backtick` here</code> carefully.</p>", new ConversionOptions());

```
