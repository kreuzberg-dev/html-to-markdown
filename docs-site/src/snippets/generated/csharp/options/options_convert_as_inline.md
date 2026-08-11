---
id: fixture_csharp_options_convert_as_inline
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>One</p><p>Two</p>", new ConversionOptions { ConvertAsInline = true });

```
