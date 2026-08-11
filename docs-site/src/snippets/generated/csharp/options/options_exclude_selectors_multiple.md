---
id: fixture_csharp_options_exclude_selectors_multiple
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<body><nav class=\"nav\">Menu</nav><p>Content</p><footer>Footer</footer></body>", new ConversionOptions { ExcludeSelectors = new List<string> { ".nav", "footer" } });

```
