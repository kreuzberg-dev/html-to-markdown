---
id: fixture_csharp_conversion_autolink_filename_not_autolinked
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<a href=\"foobar.png\">foobar.png</a>", new ConversionOptions());

```
