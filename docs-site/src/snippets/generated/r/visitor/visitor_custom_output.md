```r title="R"
library("htmltomarkdown", character.only = TRUE)

visitor <- list(
  visit_heading = function(ctx, level, text, id) {
    list(Custom = "## REPLACED HEADING")
  }
)

result <- convert(html = "<h1>Original Heading</h1>", options = list(visitor = visitor))

```
