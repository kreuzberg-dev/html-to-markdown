---
id: fixture_csharp_options_exclude_selectors_empty_noop
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>Hello world</p>", new ConversionOptions { ExcludeSelectors = new List<string> {  } });

```
