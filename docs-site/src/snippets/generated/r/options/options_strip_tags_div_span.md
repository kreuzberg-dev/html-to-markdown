---
id: fixture_r_options_strip_tags_div_span
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<div class='wrapper'><p>Inside div</p></div><p>Outside <span class='hl'>span text</span></p>", options = ConversionOptions$from_json(jsonlite::toJSON(list("strip_tags" = I(c("div", "span"))), auto_unbox = TRUE)))

```
