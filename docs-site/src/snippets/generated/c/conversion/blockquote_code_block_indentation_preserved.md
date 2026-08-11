---
id: fixture_c_blockquote_code_block_indentation_preserved
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
    HTMConversionResult* result = htm_convert("<blockquote><pre><code>line1\n    line2 indented</code></pre></blockquote>", NULL);
    htm_conversion_result_free(result);
    return EXIT_SUCCESS;
}

```
