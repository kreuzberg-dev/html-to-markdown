---
id: fixture_r_options_exclude_selectors_class
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<body><div class=\"cookie-banner\">Accept cookies</div><p>Main content</p></body>", options = ConversionOptions$from_json(jsonlite::toJSON(list("exclude_selectors" = I(c(".cookie-banner"))), auto_unbox = TRUE)))

```
