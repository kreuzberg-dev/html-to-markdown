---
id: fixture_csharp_blockquote_code_block_indentation_preserved
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<blockquote><pre><code>line1\n    line2 indented</code></pre></blockquote>", new ConversionOptions());

```
