```r title="R"
library("htmltomarkdown", character.only = TRUE)

visitor <- list(
  visit_custom_element = function(ctx, tag_name, html) {
    list(Custom = "[CUSTOM WIDGET]")
  }
)

result <- convert(html = "<div><custom-widget data-value=\"123\"><p>Widget content here</p><span>With nested elements</span></custom-widget></div>", options = list(visitor = visitor))

```
