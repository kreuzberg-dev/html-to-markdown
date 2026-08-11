---
id: fixture_r_visitor_skip_code_blocks
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

visitor <- list(
  visit_code_block = function(ctx, lang, code) {
    "Skip"
  }
)

result <- convert(html = "<p>Intro text</p><pre><code>let x = 42;</code></pre><p>Outro text</p>", options = list(visitor = visitor))

```
