---
id: fixture_csharp_result_warnings_empty_for_clean_input
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<h1>Title</h1><p>Clean content with <a href='https://example.com'>a link</a>.</p>", new ConversionOptions());

```
