---
id: fixture_csharp_visitor_form_skip
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>Before form</p><form><input type=\"email\" name=\"email\"></form><p>After form</p>", new ConversionOptions());

```
