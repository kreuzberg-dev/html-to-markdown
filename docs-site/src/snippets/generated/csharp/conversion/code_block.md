---
id: fixture_csharp_code_block
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<pre><code class=\"language-python\">print('hello')</code></pre>", new ConversionOptions());

```
