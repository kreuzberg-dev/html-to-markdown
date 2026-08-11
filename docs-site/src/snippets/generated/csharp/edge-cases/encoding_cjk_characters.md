---
id: fixture_csharp_encoding_cjk_characters
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>中文内容</p><p>日本語テキスト</p><p>한국어 텍스트</p>", new ConversionOptions());

```
