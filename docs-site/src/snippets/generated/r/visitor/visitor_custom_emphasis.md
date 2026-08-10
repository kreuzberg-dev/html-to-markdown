```r title="R"
library("htmltomarkdown", character.only = TRUE)

visitor <- list(
  visit_emphasis = function(ctx, text) {
    list(Custom = paste0(">>>", text, "<<<"))
  }
)

result <- convert(html = "<p>This is <em>important</em> text.</p>", options = list(visitor = visitor))

```
