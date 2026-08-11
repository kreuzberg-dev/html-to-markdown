---
id: fixture_r_semantic_abbr
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>The <abbr title=\"World Wide Web\">WWW</abbr> is global.</p>", options = ConversionOptions$default())

```
