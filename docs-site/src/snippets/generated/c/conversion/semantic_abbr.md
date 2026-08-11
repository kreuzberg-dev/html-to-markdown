---
id: fixture_c_semantic_abbr
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
    HTMConversionResult* result = htm_convert("<p>The <abbr title=\"World Wide Web\">WWW</abbr> is global.</p>", NULL);
    htm_conversion_result_free(result);
    return EXIT_SUCCESS;
}

```
