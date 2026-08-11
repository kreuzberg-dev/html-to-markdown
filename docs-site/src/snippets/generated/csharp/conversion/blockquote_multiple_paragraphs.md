---
id: fixture_csharp_blockquote_multiple_paragraphs
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<blockquote><p>First paragraph.</p><p>Second paragraph.</p></blockquote>", new ConversionOptions());

```
