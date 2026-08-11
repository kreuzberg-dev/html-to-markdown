---
id: fixture_csharp_metadata_image_type_external_classified
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p><img src=\"https://example.com/photo.jpg\" alt=\"A photo\"></p>", new ConversionOptions { ExtractMetadata = true });

```
