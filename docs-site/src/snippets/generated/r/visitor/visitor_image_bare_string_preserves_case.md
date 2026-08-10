```r title="R"
library("htmltomarkdown", character.only = TRUE)

visitor <- list(
  visit_image = function(ctx, src, alt, title) {
    paste0("[image: ", alt, " -> ", src, "]")
  }
)

result <- convert(html = "<img src=\"PhotoOne.JPG\" alt=\"Sunset Over Bay\">", options = list(visitor = visitor))

```
