---
id: fixture_csharp_options_preserve_tags_iframe
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>Before</p><iframe src='video.html' width='560'></iframe><p>After</p>", new ConversionOptions { PreserveTags = new List<string> { "iframe" } });

```
