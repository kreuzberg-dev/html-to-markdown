---
id: fixture_csharp_options_escape_ascii_enabled
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>Text with # hash and [brackets] and * star</p>", new ConversionOptions { EscapeAscii = true });

```
