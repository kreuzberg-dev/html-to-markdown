---
id: fixture_csharp_result_warning_kind_image_extraction_failed
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>Text<img src=\"data:BADMIME\" alt=\"broken\">end</p>", new ConversionOptions { ExtractImages = true });

```
