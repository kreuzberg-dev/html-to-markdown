---
id: fixture_c_hidden_content_visibility_hidden_dropped
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
    HTMConversionResult* result = htm_convert("<p>visible</p><span style=\"visibility:hidden\">secret hidden span</span><p>also visible</p>", NULL);
    htm_conversion_result_free(result);
    return EXIT_SUCCESS;
}

```
