```r title="R"
library("htmltomarkdown", character.only = TRUE)

visitor <- list(
  visit_video = function(ctx, src) {
    "Skip"
  }
)

result <- convert(html = "<h2>Demo</h2><video src=\"demo.webm\"></video><p>See the demo above.</p>", options = list(visitor = visitor))

```
