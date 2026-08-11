---
id: fixture_csharp_metadata_link_type_external_classified
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>See <a href=\"https://example.com\">Example</a> for details.</p>", new ConversionOptions { ExtractMetadata = true });

```
