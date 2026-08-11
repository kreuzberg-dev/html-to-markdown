---
id: fixture_csharp_metadata_title_tag
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<html><head><title>My Page</title></head><body><p>Content</p></body></html>", new ConversionOptions { ExtractMetadata = true });

```
