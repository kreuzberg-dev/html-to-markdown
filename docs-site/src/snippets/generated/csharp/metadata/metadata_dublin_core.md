---
id: fixture_csharp_metadata_dublin_core
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<html><head><title>Scholarly Work</title><meta name=\"DC.title\" content=\"Principles of Knowledge Management\"><meta name=\"DC.creator\" content=\"Dr. Alice Johnson\"><meta name=\"DC.date\" content=\"2023-06-15\"><meta name=\"DC.subject\" content=\"Knowledge Management\"><meta name=\"DC.publisher\" content=\"Academic Press\"></head><body><p>This is a scholarly article.</p></body></html>", new ConversionOptions { ExtractMetadata = true });

```
