```r title="R"
library("htmltomarkdown", character.only = TRUE)

visitor <- list(
  visit_custom_element = function(ctx, tag_name, html) {
    "PreserveHtml"
  }
)

result <- convert(html = "<div><custom-tag>Custom content</custom-tag></div>", options = list(visitor = visitor))

```
