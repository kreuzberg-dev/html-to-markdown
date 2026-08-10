```r title="R"
library("htmltomarkdown", character.only = TRUE)

visitor <- list(
  visit_image = function(ctx, src, alt, title) {
    list(Custom = paste0("[Image: ", alt, "]"))
  }
)

result <- convert(html = "<img src=\"banner.png\" alt=\"Banner\">", options = list(visitor = visitor))

```
