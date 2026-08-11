---
id: fixture_csharp_link_with_bold_text
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<a href=\"https://example.com\"><strong>Bold link</strong></a>", new ConversionOptions());

```
