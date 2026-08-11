---
id: fixture_csharp_options_exclude_selectors_plain_text_mode
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<body><div class=\"nav\">Navigation</div><p>Article body</p></body>", new ConversionOptions { ExcludeSelectors = new List<string> { ".nav" }, OutputFormat = OutputFormat.Plain });

```
