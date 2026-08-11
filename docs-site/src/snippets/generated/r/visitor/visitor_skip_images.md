---
id: fixture_r_visitor_skip_images
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

visitor <- list(
  visit_image = function(ctx, src, alt, title) {
    "Skip"
  }
)

result <- convert(html = "<p>Before image</p><img src=\"photo.jpg\" alt=\"A photo\"><p>After image</p>", options = list(visitor = visitor))

```
