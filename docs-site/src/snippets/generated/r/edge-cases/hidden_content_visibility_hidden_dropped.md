---
id: fixture_r_hidden_content_visibility_hidden_dropped
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>visible</p><span style=\"visibility:hidden\">secret hidden span</span><p>also visible</p>", options = ConversionOptions$default())

```
