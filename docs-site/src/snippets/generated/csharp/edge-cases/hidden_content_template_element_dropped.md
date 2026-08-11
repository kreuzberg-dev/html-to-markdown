---
id: fixture_csharp_hidden_content_template_element_dropped
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>visible</p><template><p>secret template text</p></template><p>also visible</p>", new ConversionOptions());

```
