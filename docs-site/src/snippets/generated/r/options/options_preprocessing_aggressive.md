```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<nav>Menu</nav><article><h1>Title</h1><p>Content</p></article><aside>Sidebar</aside><footer>Footer</footer>", options = ConversionOptions$from_json(jsonlite::toJSON(list("preprocessing" = list("preset" = "aggressive")), auto_unbox = TRUE)))

```
