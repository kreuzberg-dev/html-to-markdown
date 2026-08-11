---
id: fixture_r_hidden_content_template_element_dropped
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>visible</p><template><p>secret template text</p></template><p>also visible</p>", options = ConversionOptions$default())

```
