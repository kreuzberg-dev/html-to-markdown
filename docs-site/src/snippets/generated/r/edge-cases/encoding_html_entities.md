---
id: fixture_r_encoding_html_entities
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>&amp; &lt; &gt; &nbsp; &quot; &apos;</p>", options = ConversionOptions$default())

```
