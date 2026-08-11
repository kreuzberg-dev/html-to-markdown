---
id: fixture_r_options_exclude_selectors_id
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<body><div id=\"ad-container\">Buy stuff</div><p>Article text</p></body>", options = ConversionOptions$from_json(jsonlite::toJSON(list("exclude_selectors" = I(c("#ad-container"))), auto_unbox = TRUE)))

```
