---
id: fixture_c_smoke_simple_heading
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
    HTMConversionResult* result = htm_convert("<h1>Title</h1>", NULL);
    htm_conversion_result_free(result);
    return EXIT_SUCCESS;
}

```
