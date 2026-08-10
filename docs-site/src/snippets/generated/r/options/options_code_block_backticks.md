```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<pre><code class=\"language-js\">console.log('hi');</code></pre>", options = ConversionOptions$from_json(jsonlite::toJSON(list("code_block_style" = "backticks"), auto_unbox = TRUE)))

```
