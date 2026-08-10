```r title="R"
library("htmltomarkdown", character.only = TRUE)

visitor <- list(
  visit_definition_description = function(ctx, text) {
    "Skip"
  },
  visit_definition_term = function(ctx, text) {
    "Skip"
  }
)

result <- convert(html = "<p>Glossary:</p><dl><dt>Term A</dt><dd>Definition of term A</dd><dt>Term B</dt><dd>Definition of term B</dd></dl><p>End of glossary</p>", options = list(visitor = visitor))

```
