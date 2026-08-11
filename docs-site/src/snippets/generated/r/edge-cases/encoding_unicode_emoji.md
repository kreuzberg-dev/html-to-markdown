---
id: fixture_r_encoding_unicode_emoji
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>Hello 🌍 World 🚀</p><p>Stars: ⭐ ✨</p>", options = ConversionOptions$default())

```
