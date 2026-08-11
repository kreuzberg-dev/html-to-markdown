---
id: fixture_r_line_break_multiple_br
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>Start.<br><br>End.</p>", options = ConversionOptions$default())

```
