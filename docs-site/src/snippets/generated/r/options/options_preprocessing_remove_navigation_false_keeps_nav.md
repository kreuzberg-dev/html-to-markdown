---
id: fixture_r_options_preprocessing_remove_navigation_false_keeps_nav
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<nav>SiteMenu</nav><main><p>MainContent</p></main><aside>SidebarText</aside>", options = ConversionOptions$from_json(jsonlite::toJSON(list("preprocessing" = list("remove_navigation" = FALSE)), auto_unbox = TRUE)))

```
