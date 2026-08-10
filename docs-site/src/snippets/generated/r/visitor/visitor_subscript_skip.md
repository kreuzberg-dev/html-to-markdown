```r title="R"
library("htmltomarkdown", character.only = TRUE)

visitor <- list(
  visit_subscript = function(ctx, text) {
    "Skip"
  }
)

result <- convert(html = "<p>The formula C<sub>12</sub>H<sub>22</sub>O<sub>11</sub> is sugar.</p>", options = list(visitor = visitor))

```
