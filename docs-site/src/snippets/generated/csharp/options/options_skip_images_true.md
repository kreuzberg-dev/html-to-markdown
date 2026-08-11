---
id: fixture_csharp_options_skip_images_true
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>Before <img src='test.jpg' alt='photo'> After</p>", new ConversionOptions { SkipImages = true });

```
