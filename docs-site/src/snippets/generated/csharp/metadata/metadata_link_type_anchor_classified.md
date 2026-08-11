---
id: fixture_csharp_metadata_link_type_anchor_classified
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>Jump to <a href=\"#section\">section</a> below.</p>", new ConversionOptions { ExtractMetadata = true });

```
