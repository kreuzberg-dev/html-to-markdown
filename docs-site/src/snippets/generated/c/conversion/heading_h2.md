```c title="C"
#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "html_to_markdown.h"

int main(void) {
    HTMConversionResult* result = htm_convert("<h2>Heading 2</h2>", NULL);
    htm_conversion_result_free(result);
    return EXIT_SUCCESS;
}

```
