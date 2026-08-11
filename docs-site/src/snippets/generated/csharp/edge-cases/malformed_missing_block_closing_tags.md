---
id: fixture_csharp_malformed_missing_block_closing_tags
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<div><h1>Title<p>First paragraph<p>Second paragraph</div>", new ConversionOptions());

```
