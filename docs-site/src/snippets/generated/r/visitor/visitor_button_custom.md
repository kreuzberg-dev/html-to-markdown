---
id: fixture_r_visitor_button_custom
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

visitor <- list(
  visit_button = function(ctx, text) {
    list(Custom = paste0("[BTN:", text, "]"))
  }
)

result <- convert(html = "<p>Confirm action: <button type=\"submit\">Click me</button> or <button type=\"reset\">Cancel</button></p>", options = list(visitor = visitor))

```
