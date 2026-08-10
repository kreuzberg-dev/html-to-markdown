```r title="R"
library("htmltomarkdown", character.only = TRUE)

visitor <- list(
  visit_input = function(ctx, input_type, name, value) {
    list(Custom = paste0("[INPUT:", input_type, "]"))
  }
)

result <- convert(html = "<form><label>Username: <input type=\"text\" name=\"username\" value=\"\"></label><label>Password: <input type=\"password\" name=\"password\"></label></form>", options = list(visitor = visitor))

```
