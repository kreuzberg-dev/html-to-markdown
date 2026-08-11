---
id: fixture_csharp_code_block_no_language
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<pre><code>plain code here</code></pre>", new ConversionOptions());

```
