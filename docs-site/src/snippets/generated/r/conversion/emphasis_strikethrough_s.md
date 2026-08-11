---
id: fixture_r_emphasis_strikethrough_s
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p><s>strikethrough</s></p>", options = ConversionOptions$default())

```
