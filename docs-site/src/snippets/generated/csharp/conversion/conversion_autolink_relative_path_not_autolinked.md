---
id: fixture_csharp_conversion_autolink_relative_path_not_autolinked
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<a href=\"/docs/intro.html\">/docs/intro.html</a>", new ConversionOptions());

```
