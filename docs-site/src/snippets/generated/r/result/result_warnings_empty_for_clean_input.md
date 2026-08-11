---
id: fixture_r_result_warnings_empty_for_clean_input
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<h1>Title</h1><p>Clean content with <a href='https://example.com'>a link</a>.</p>", options = ConversionOptions$default())

```
