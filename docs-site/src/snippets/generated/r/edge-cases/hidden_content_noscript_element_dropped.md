---
id: fixture_r_hidden_content_noscript_element_dropped
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>visible</p><noscript><p>secret noscript text</p></noscript><p>also visible</p>", options = ConversionOptions$default())

```
