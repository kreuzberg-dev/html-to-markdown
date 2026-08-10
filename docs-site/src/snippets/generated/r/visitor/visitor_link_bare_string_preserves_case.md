```r title="R"
library("htmltomarkdown", character.only = TRUE)

visitor <- list(
  visit_link = function(ctx, href, text, title) {
    paste0("[", text, "](https://new-cdn.com/file.pdf)")
  }
)

result <- convert(html = "<a href=\"https://old-cdn.com/file.pdf\">Download</a>", options = list(visitor = visitor))

```
