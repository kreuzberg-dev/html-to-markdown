---
id: fixture_c_issue_396_backticks_blank_line_after_fence
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
    HTMConversionOptions* options_handle = htm_conversion_options_from_json("{\"code_block_style\":\"Backticks\"}");
    HTMConversionResult* result = htm_convert("<p>Foo</p><pre><code>1\n2\n</code></pre><p>Bar</p>", options_handle);
    htm_conversion_options_free(options_handle);
    htm_conversion_result_free(result);
    return EXIT_SUCCESS;
}

```
