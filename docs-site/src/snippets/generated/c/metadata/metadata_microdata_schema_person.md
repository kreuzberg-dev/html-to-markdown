---
id: fixture_c_metadata_microdata_schema_person
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
    HTMConversionResult* result = htm_convert("<html><head><title>Contact</title></head><body><div itemscope itemtype=\"https://schema.org/Person\"><span itemprop=\"name\">John Smith</span><span itemprop=\"email\">john@example.com</span><span itemprop=\"telephone\">+1-555-0100</span></div></body></html>", options_handle);
    htm_conversion_options_free(options_handle);
    htm_conversion_result_free(result);
    return EXIT_SUCCESS;
}

```
