```r
library(htmltomarkdown)

html <- "
<table>
    <tr><th>Name</th><th>Age</th></tr>
    <tr><td>Alice</td><td>30</td></tr>
    <tr><td>Bob</td><td>25</td></tr>
</table>
"

# `tables` is collected alongside the document tree, so it must be enabled.
opts <- conversion_options(include_document_structure = TRUE)
result <- convert(html, opts)

for (table in result$tables) {
  for (cell in table$grid$cells) {
    prefix <- if (cell$is_header) "Header" else "Cell"
    cat(sprintf("  %s (r%d,c%d): %s\n", prefix, cell$row, cell$col, cell$content))
  }
}
```
