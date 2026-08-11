---
id: fixture_r_hidden_content_aria_hidden_still_rendered
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>visible</p><div aria-hidden=\"true\">still shown</div><p>also visible</p>", options = ConversionOptions$default())

```
