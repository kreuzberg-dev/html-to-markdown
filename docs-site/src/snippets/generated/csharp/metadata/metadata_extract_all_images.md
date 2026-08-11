---
id: fixture_csharp_metadata_extract_all_images
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<html><head><title>Gallery</title></head><body><img src=\"https://example.com/photo1.jpg\" alt=\"Photo 1\"><img src=\"https://example.com/photo2.png\" alt=\"Photo 2\"><img src=\"/local/image.webp\" alt=\"Local image\"></body></html>", new ConversionOptions { ExtractMetadata = true });

```
