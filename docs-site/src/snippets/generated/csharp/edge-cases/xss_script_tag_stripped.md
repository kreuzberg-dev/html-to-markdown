---
id: fixture_csharp_xss_script_tag_stripped
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>Safe content.</p><script>alert('xss')</script><p>More safe content.</p>", new ConversionOptions());

```
