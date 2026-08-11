---
id: fixture_r_visitor_audio_custom
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
    list(Custom = "[AUDIO: podcast.mp3]")
  }
)

result <- convert(html = "<p>Listen to this: <audio src=\"podcast.mp3\" controls></audio></p>", options = list(visitor = visitor))

```
