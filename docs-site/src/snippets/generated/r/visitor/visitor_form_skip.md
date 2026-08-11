---
id: fixture_r_visitor_form_skip
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

visitor <- list(
  visit_form = function(ctx, action_url, method) {
    "Skip"
  }
)

result <- convert(html = "<p>Before form</p><form><input type=\"email\" name=\"email\"></form><p>After form</p>", options = list(visitor = visitor))

```
