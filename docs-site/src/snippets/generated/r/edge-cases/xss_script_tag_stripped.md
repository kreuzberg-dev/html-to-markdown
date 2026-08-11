---
id: fixture_r_xss_script_tag_stripped
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>Safe content.</p><script>alert('xss')</script><p>More safe content.</p>", options = ConversionOptions$default())

```
