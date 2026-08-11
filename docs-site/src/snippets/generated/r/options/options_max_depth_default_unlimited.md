---
id: fixture_r_options_max_depth_default_unlimited
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<div><div><div><div><p>Deep content</p></div></div></div></div>", options = ConversionOptions$default())

```
