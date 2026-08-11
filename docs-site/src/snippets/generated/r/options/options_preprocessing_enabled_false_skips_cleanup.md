---
id: fixture_r_options_preprocessing_enabled_false_skips_cleanup
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<nav>NavSection</nav><p>Paragraph</p>", options = ConversionOptions$from_json(jsonlite::toJSON(list("preprocessing" = list("enabled" = FALSE)), auto_unbox = TRUE)))

```
