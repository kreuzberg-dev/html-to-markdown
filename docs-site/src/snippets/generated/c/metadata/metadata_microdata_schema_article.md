---
id: fixture_c_metadata_microdata_schema_article
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
    HTMConversionOptions* options_handle = htm_conversion_options_from_json("{\"extract_metadata\":true}");
    HTMConversionResult* result = htm_convert("<html><head><title>Article</title></head><body><article itemscope itemtype=\"https://schema.org/Article\"><h1 itemprop=\"headline\">Breaking News Today</h1><span itemprop=\"author\">Jane Reporter</span><span itemprop=\"datePublished\">2024-04-22</span><div itemprop=\"articleBody\"><p>The article content goes here with important information about the breaking news story.</p></div></article></body></html>", options_handle);
    htm_conversion_options_free(options_handle);
    htm_conversion_result_free(result);
    return EXIT_SUCCESS;
}

```
