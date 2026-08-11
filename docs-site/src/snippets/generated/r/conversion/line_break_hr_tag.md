---
id: fixture_r_line_break_hr_tag
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>Before rule.</p><hr><p>After rule.</p>", options = ConversionOptions$default())

```
