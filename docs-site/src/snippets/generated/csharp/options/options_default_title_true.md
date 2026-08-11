---
id: fixture_csharp_options_default_title_true
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p><a href='https://example.com'>Link</a></p>", new ConversionOptions { DefaultTitle = true });

```
