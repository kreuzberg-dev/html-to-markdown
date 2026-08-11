---
id: fixture_csharp_options_exclude_selectors_attribute
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<body><div role=\"complementary\">Sidebar</div><p>Primary text</p></body>", new ConversionOptions { ExcludeSelectors = new List<string> { "[role='complementary']" } });

```
