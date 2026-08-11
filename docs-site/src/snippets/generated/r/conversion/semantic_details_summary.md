---
id: fixture_r_semantic_details_summary
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<details><summary>Click to expand</summary><p>Hidden content here.</p></details>", options = ConversionOptions$default())

```
