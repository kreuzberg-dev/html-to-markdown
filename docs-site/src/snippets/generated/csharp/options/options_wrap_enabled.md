---
id: fixture_csharp_options_wrap_enabled
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>This is a long paragraph that should be wrapped at the specified column width when the wrap option is enabled.</p>", new ConversionOptions { Wrap = true, WrapWidth = 40 });

```
