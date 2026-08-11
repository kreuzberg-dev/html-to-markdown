---
id: fixture_csharp_form_textarea
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<form><label>Message:</label><textarea>Default text content</textarea></form>", new ConversionOptions { Preprocessing = new PreprocessingOptions { RemoveForms = false } });

```
