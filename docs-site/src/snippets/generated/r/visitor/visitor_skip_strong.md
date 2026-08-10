```r title="R"
library("htmltomarkdown", character.only = TRUE)

visitor <- list(
  visit_strong = function(ctx, text) {
    "Skip"
  }
)

result <- convert(html = "<p>Normal <strong>bold text</strong> normal</p>", options = list(visitor = visitor))

```
