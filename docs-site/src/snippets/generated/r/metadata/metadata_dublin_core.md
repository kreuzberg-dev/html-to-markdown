```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<html><head><title>Scholarly Work</title><meta name=\"DC.title\" content=\"Principles of Knowledge Management\"><meta name=\"DC.creator\" content=\"Dr. Alice Johnson\"><meta name=\"DC.date\" content=\"2023-06-15\"><meta name=\"DC.subject\" content=\"Knowledge Management\"><meta name=\"DC.publisher\" content=\"Academic Press\"></head><body><p>This is a scholarly article.</p></body></html>", options = ConversionOptions$from_json(jsonlite::toJSON(list("extract_metadata" = TRUE), auto_unbox = TRUE)))

```
