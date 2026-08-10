```r title="R"
library("htmltomarkdown", character.only = TRUE)

visitor <- list(
  visit_heading = function(ctx, level, text, id) {
    paste0("## ", text, " ##")
  }
)

result <- convert(html = "<h2>Important Section Title</h2><p>Body.</p>", options = list(visitor = visitor))

```
