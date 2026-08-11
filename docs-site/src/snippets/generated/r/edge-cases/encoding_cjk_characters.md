---
id: fixture_r_encoding_cjk_characters
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>中文内容</p><p>日本語テキスト</p><p>한국어 텍스트</p>", options = ConversionOptions$default())

```
