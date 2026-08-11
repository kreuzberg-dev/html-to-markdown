---
id: fixture_csharp_options_preprocessing_enabled_false_skips_cleanup
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<nav>NavSection</nav><p>Paragraph</p>", new ConversionOptions { Preprocessing = new PreprocessingOptions { Enabled = false } });

```
