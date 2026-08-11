---
id: fixture_r_options_keep_inline_images_in_paragraph
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>Text <img src='icon.png' alt='icon'> more text</p>", options = ConversionOptions$from_json(jsonlite::toJSON(list("keep_inline_images_in" = I(c("p"))), auto_unbox = TRUE)))

```
