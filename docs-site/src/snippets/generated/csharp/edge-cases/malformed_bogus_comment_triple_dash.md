---
id: fixture_csharp_malformed_bogus_comment_triple_dash
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<h1>One</h1>\n<!-- /// --->\n<p>Two</p>", new ConversionOptions());

```
