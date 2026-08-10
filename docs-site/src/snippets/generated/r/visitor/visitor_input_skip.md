```r title="R"
library("htmltomarkdown", character.only = TRUE)

visitor <- list(
  visit_input = function(ctx, input_type, name, value) {
    "Skip"
  }
)

result <- convert(html = "<p>Sign up:</p><input type=\"text\" name=\"email\" placeholder=\"your@email.com\"><input type=\"checkbox\" name=\"agree\"><p>Continue</p>", options = list(visitor = visitor))

```
