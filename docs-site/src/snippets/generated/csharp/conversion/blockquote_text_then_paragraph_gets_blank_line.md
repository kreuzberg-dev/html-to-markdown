---
id: fixture_csharp_blockquote_text_then_paragraph_gets_blank_line
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<blockquote>Just text, then <p>a paragraph</p></blockquote>", new ConversionOptions());

```
