---
id: fixture_csharp_visitor_audio_custom
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>Listen to this: <audio src=\"podcast.mp3\" controls></audio></p>", new ConversionOptions());

```
