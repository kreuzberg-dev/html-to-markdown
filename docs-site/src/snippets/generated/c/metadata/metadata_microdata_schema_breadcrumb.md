---
id: fixture_c_metadata_microdata_schema_breadcrumb
language: c
target: c
level: typecheck
requires: []
side_effect: safe
---

```c title="C"
#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "html_to_markdown.h"

int main(void) {
    HTMConversionOptions* options_handle = htm_conversion_options_from_json("{\"extract_metadata\":true,\"preprocessing\":{\"remove_navigation\":false}}");
    HTMConversionResult* result = htm_convert("<html><head><title>Navigation</title></head><body><nav itemscope itemtype=\"https://schema.org/BreadcrumbList\"><span itemprop=\"itemListElement\" itemscope itemtype=\"https://schema.org/ListItem\"><a itemprop=\"item\" href=\"https://example.com\"><span itemprop=\"name\">Home</span></a></span><span itemprop=\"itemListElement\" itemscope itemtype=\"https://schema.org/ListItem\"><a itemprop=\"item\" href=\"https://example.com/products\"><span itemprop=\"name\">Products</span></a></span><span itemprop=\"itemListElement\" itemscope itemtype=\"https://schema.org/ListItem\"><span itemprop=\"name\">Current Page</span></span></nav></body></html>", options_handle);
    htm_conversion_options_free(options_handle);
    htm_conversion_result_free(result);
    return EXIT_SUCCESS;
}

```
