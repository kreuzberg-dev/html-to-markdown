```c title="C"
#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "html_to_markdown.h"

int main(void) {
    HTMConversionResult* result = htm_convert("<ol><li>Step 1<ol><li>Step 1a</li><li>Step 1b</li></ol></li><li>Step 2</li></ol>", NULL);
    htm_conversion_result_free(result);
    return EXIT_SUCCESS;
}

```
