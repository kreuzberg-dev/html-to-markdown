---
id: fixture_r_visitor_definition_list_custom_format
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

visitor <- list(
  visit_definition_description = function(ctx, text) {
    list(Custom = paste0("> ", text))
  },
  visit_definition_term = function(ctx, text) {
    list(Custom = paste0("### ", text))
  }
)

result <- convert(html = "<dl><dt>Python</dt><dd>A high-level programming language</dd><dt>JavaScript</dt><dd>A scripting language for web browsers</dd></dl>", options = list(visitor = visitor))

```
