---
id: fixture_r_visitor_audio_skip
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

visitor <- list(
  visit_audio = function(ctx, src) {
    "Skip"
  }
)

result <- convert(html = "<p>Background music:</p><audio src=\"music.ogg\" autoplay></audio><p>Enjoy!</p>", options = list(visitor = visitor))

```
