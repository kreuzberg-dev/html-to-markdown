---
id: fixture_csharp_options_code_language_python
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<pre><code>def hello(): pass</code></pre>", new ConversionOptions { CodeLanguage = "python" });

```
