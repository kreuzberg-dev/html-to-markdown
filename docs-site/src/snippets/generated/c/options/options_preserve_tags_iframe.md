---
id: fixture_c_options_preserve_tags_iframe
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
    HTMConversionOptions* options_handle = htm_conversion_options_from_json("{\"preserve_tags\":[\"iframe\"]}");
    HTMConversionResult* result = htm_convert("<p>Before</p><iframe src='video.html' width='560'></iframe><p>After</p>", options_handle);
    htm_conversion_options_free(options_handle);
    htm_conversion_result_free(result);
    return EXIT_SUCCESS;
}

```
