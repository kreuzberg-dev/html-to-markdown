```r
library(htmltomarkdown)

# Binary data (detected via magic bytes) is rejected before parsing.
html <- "%PDF-1.4 not actually HTML"

result <- tryCatch(
  convert(html),
  error = function(e) {
    message("conversion failed: ", conditionMessage(e))
    NULL
  }
)

if (!is.null(result)) {
  cat(result$content)
}
```
