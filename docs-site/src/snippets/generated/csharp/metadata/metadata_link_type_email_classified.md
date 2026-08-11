---
id: fixture_csharp_metadata_link_type_email_classified
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>Contact <a href=\"mailto:hello@example.com\">us</a> directly.</p>", new ConversionOptions { ExtractMetadata = true });

```
