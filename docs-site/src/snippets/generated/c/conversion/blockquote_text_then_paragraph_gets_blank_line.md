---
id: fixture_c_blockquote_text_then_paragraph_gets_blank_line
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
    HTMConversionResult* result = htm_convert("<blockquote>Just text, then <p>a paragraph</p></blockquote>", NULL);
    htm_conversion_result_free(result);
    return EXIT_SUCCESS;
}

```
