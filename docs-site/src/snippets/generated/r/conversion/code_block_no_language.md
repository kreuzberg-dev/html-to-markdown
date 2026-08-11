---
id: fixture_r_code_block_no_language
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<pre><code>plain code here</code></pre>", options = ConversionOptions$default())

```
