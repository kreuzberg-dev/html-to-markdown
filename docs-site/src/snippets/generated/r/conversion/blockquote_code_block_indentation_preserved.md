---
id: fixture_r_blockquote_code_block_indentation_preserved
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<blockquote><pre><code>line1\n    line2 indented</code></pre></blockquote>", options = ConversionOptions$default())

```
