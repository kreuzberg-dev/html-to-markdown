---
id: fixture_csharp_visitor_skip_images
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>Before image</p><img src=\"photo.jpg\" alt=\"A photo\"><p>After image</p>", new ConversionOptions());

```
