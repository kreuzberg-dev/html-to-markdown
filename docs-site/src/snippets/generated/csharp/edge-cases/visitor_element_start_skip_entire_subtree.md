---
id: fixture_csharp_visitor_element_start_skip_entire_subtree
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<div><h1>Title</h1><p>Content</p></div>", new ConversionOptions());

```
