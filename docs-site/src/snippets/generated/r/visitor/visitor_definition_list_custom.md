```r title="R"
library("htmltomarkdown", character.only = TRUE)

visitor <- list(
  visit_definition_term = function(ctx, text) {
    list(Custom = paste0("**", text, "**"))
  }
)

result <- convert(html = "<dl><dt>HTML</dt><dd>HyperText Markup Language</dd><dt>CSS</dt><dd>Cascading Style Sheets</dd></dl>", options = list(visitor = visitor))

```
