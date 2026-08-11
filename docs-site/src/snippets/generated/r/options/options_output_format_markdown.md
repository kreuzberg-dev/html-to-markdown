---
id: fixture_r_options_output_format_markdown
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<h1>Title</h1><p>Some text.</p>", options = ConversionOptions$from_json(jsonlite::toJSON(list("heading_style" = "atx", "output_format" = "markdown"), auto_unbox = TRUE)))

```
