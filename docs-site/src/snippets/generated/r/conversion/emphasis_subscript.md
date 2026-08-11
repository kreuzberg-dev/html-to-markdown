---
id: fixture_r_emphasis_subscript
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>H<sub>2</sub>O</p>", options = ConversionOptions$default())

```
