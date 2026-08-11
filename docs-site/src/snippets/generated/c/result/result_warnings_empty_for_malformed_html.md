---
id: fixture_c_result_warnings_empty_for_malformed_html
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
    HTMConversionResult* result = htm_convert("<p>Unclosed paragraph<div>Mixed nesting</p></div>", NULL);
    htm_conversion_result_free(result);
    return EXIT_SUCCESS;
}

```
