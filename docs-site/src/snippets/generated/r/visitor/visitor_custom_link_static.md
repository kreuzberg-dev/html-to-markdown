```r title="R"
library("htmltomarkdown", character.only = TRUE)

visitor <- list(
  visit_link = function(ctx, href, text, title) {
    list(Custom = "[REDACTED LINK]")
  }
)

result <- convert(html = "<a href=\"https://example.com\">Click here</a>", options = list(visitor = visitor))

```
