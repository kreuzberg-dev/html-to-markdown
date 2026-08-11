---
id: fixture_c_link_with_title
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
    HTMConversionResult* result = htm_convert("<a href=\"https://example.com\" title=\"Example Site\">Example</a>", NULL);
    htm_conversion_result_free(result);
    return EXIT_SUCCESS;
}

```
