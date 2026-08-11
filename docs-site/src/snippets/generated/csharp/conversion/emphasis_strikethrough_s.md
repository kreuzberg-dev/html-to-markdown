---
id: fixture_csharp_emphasis_strikethrough_s
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p><s>strikethrough</s></p>", new ConversionOptions());

```
