```r title="R"
library("htmltomarkdown", character.only = TRUE)

visitor <- list(
  visit_mark = function(ctx, text) {
    "Skip"
  }
)

result <- convert(html = "<p>Key insight: <mark>always validate input</mark> for security.</p>", options = list(visitor = visitor))

```
