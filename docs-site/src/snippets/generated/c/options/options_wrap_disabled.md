---
id: fixture_c_options_wrap_disabled
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
    HTMConversionOptions* options_handle = htm_conversion_options_from_json("{\"wrap\":false}");
    HTMConversionResult* result = htm_convert("<p>This is a long paragraph that should not be wrapped at all because wrapping is disabled.</p>", options_handle);
    htm_conversion_options_free(options_handle);
    htm_conversion_result_free(result);
    return EXIT_SUCCESS;
}

```
