```r title="R"
library("htmltomarkdown", character.only = TRUE)

visitor <- list(
  visit_underline = function(ctx, text) {
    list(Custom = paste0("_", text, "_"))
  }
)

result <- convert(html = "<p>This is <u>very important</u> text.</p>", options = list(visitor = visitor))

```
