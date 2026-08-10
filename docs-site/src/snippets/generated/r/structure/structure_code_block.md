```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>Example code:</p><pre><code class=\"language-rust\">fn main() { println!(\"Hello\"); }</code></pre>", options = ConversionOptions$from_json(jsonlite::toJSON(list("include_document_structure" = TRUE), auto_unbox = TRUE)))

```
