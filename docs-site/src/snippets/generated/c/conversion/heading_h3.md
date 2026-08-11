---
id: fixture_c_heading_h3
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
    HTMConversionResult* result = htm_convert("<h3>Heading 3</h3>", NULL);
    htm_conversion_result_free(result);
    return EXIT_SUCCESS;
}

```
