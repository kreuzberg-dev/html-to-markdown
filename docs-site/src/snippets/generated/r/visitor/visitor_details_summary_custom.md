```r title="R"
library("htmltomarkdown", character.only = TRUE)

visitor <- list(
  visit_summary = function(ctx, text) {
    list(Custom = paste0("[EXPANDABLE] ", text))
  }
)

result <- convert(html = "<details><summary>Click to expand</summary><p>This content is initially hidden.</p><p>But can be revealed by the user.</p></details>", options = list(visitor = visitor))

```
