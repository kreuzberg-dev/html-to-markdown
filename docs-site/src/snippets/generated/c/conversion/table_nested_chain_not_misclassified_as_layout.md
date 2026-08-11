---
id: fixture_c_table_nested_chain_not_misclassified_as_layout
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
    HTMConversionResult* result = htm_convert("<table><tr><td><table><tr><td><table><tr><td>leaf</td></tr></table></td></tr></table></td></tr></table>", NULL);
    htm_conversion_result_free(result);
    return EXIT_SUCCESS;
}

```
