---
id: fixture_c_semantic_section_with_heading
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
    HTMConversionResult* result = htm_convert("<section><h3>Section Heading</h3><p>Section content.</p></section>", NULL);
    htm_conversion_result_free(result);
    return EXIT_SUCCESS;
}

```
