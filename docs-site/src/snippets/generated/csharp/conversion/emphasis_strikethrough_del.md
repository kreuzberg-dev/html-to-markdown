---
id: fixture_csharp_emphasis_strikethrough_del
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p><del>deleted text</del></p>", new ConversionOptions());

```
