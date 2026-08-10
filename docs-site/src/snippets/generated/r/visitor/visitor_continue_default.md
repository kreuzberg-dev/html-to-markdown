```r title="R"
library("htmltomarkdown", character.only = TRUE)

visitor <- list(
  visit_strong = function(ctx, text) {
    "Continue"
  }
)

result <- convert(html = "<p>Hello <strong>World</strong></p>", options = list(visitor = visitor))

```
