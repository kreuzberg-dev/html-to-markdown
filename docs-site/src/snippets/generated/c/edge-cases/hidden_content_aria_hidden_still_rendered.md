---
id: fixture_c_hidden_content_aria_hidden_still_rendered
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
    HTMConversionResult* result = htm_convert("<p>visible</p><div aria-hidden=\"true\">still shown</div><p>also visible</p>", NULL);
    htm_conversion_result_free(result);
    return EXIT_SUCCESS;
}

```
