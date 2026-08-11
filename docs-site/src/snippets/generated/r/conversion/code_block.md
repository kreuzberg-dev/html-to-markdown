---
id: fixture_r_code_block
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<pre><code class=\"language-python\">print('hello')</code></pre>", options = ConversionOptions$default())

```
