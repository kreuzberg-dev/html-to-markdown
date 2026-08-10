```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>Below SVG:</p><svg xmlns=\"http://www.w3.org/2000/svg\" width=\"10\" height=\"10\"><rect width=\"10\" height=\"10\" fill=\"red\"/></svg>", options = ConversionOptions$from_json(jsonlite::toJSON(list("capture_svg" = FALSE, "extract_images" = TRUE), auto_unbox = TRUE)))

```
