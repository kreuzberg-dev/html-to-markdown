---
id: fixture_r_visitor_details_summary_skip
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

visitor <- list(
  visit_details = function(ctx, open) {
    "Skip"
  }
)

result <- convert(html = "<p>Main content here.</p><details><summary>Hidden section</summary><p>Secret details</p></details><p>More main content.</p>", options = list(visitor = visitor))

```
