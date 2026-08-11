---
id: fixture_r_malformed_missing_block_closing_tags
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<div><h1>Title<p>First paragraph<p>Second paragraph</div>", options = ConversionOptions$default())

```
