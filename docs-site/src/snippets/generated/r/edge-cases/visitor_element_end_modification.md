```r title="R"
library("htmltomarkdown", character.only = TRUE)

visitor <- list(
  visit_element_end = function(ctx, output) {
    list(Custom = "MODIFIED OUTPUT")
  }
)

result <- convert(html = "<blockquote><p>Original quote</p></blockquote>", options = list(visitor = visitor))

```
