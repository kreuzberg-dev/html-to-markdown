---
id: fixture_csharp_visitor_button_skip
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>Actions available: <button>Save</button> <button>Delete</button> <button>Export</button></p>", new ConversionOptions());

```
