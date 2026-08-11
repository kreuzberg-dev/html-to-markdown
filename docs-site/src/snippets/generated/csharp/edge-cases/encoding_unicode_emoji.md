---
id: fixture_csharp_encoding_unicode_emoji
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>Hello 🌍 World 🚀</p><p>Stars: ⭐ ✨</p>", new ConversionOptions());

```
