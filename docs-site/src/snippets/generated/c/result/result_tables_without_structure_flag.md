---
id: fixture_c_result_tables_without_structure_flag
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
    HTMConversionResult* result = htm_convert("<table><tr><th>X</th></tr><tr><td>Y</td></tr></table>", NULL);
    htm_conversion_result_free(result);
    return EXIT_SUCCESS;
}

```
