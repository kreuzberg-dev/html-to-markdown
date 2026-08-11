---
id: fixture_r_hidden_content_display_none_dropped
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>visible</p><div style=\"display:none\">secret hidden text</div><p>also visible</p>", options = ConversionOptions$default())

```
