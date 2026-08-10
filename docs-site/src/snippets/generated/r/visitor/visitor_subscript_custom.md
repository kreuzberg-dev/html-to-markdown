```r title="R"
library("htmltomarkdown", character.only = TRUE)

visitor <- list(
  visit_subscript = function(ctx, text) {
    list(Custom = paste0("~", text, "~"))
  }
)

result <- convert(html = "<p>H<sub>2</sub>O is water.</p>", options = list(visitor = visitor))

```
