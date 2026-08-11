---
id: fixture_c_options_max_depth_default_unlimited
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
    HTMConversionResult* result = htm_convert("<div><div><div><div><p>Deep content</p></div></div></div></div>", NULL);
    htm_conversion_result_free(result);
    return EXIT_SUCCESS;
}

```
