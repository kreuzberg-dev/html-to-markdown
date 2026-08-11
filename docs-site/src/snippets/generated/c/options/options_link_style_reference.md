---
id: fixture_c_options_link_style_reference
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
    HTMConversionOptions* options_handle = htm_conversion_options_from_json("{\"link_style\":\"Reference\"}");
    HTMConversionResult* result = htm_convert("<p><a href='https://example.com'>Example</a> and <a href='https://other.com'>Other</a></p>", options_handle);
    htm_conversion_options_free(options_handle);
    htm_conversion_result_free(result);
    return EXIT_SUCCESS;
}

```
