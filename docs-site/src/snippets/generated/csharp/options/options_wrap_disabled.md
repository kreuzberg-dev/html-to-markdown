---
id: fixture_csharp_options_wrap_disabled
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>This is a long paragraph that should not be wrapped at all because wrapping is disabled.</p>", new ConversionOptions { Wrap = false });

```
