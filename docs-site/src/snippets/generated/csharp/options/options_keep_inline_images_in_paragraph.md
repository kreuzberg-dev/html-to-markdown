---
id: fixture_csharp_options_keep_inline_images_in_paragraph
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>Text <img src='icon.png' alt='icon'> more text</p>", new ConversionOptions { KeepInlineImagesIn = new List<string> { "p" } });

```
