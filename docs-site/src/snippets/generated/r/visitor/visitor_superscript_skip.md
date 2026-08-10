```r title="R"
library("htmltomarkdown", character.only = TRUE)

visitor <- list(
  visit_superscript = function(ctx, text) {
    "Skip"
  }
)

result <- convert(html = "<p>The equation x<sup>3</sup> + y<sup>3</sup> = z<sup>3</sup> has no solutions.</p>", options = list(visitor = visitor))

```
