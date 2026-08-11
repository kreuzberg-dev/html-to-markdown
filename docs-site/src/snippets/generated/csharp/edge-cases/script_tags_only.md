---
id: fixture_csharp_script_tags_only
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<html><head><script>alert('xss')</script></head><body><script>document.write('hello')</script></body></html>", new ConversionOptions());

```
