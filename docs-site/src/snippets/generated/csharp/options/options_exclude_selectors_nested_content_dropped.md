---
id: fixture_csharp_options_exclude_selectors_nested_content_dropped
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<body><aside class=\"sidebar\"><h2>Related</h2><p>Sidebar text</p></aside><main><p>Main text</p></main></body>", new ConversionOptions { ExcludeSelectors = new List<string> { ".sidebar" } });

```
