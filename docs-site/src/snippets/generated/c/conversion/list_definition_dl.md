```c title="C"
#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "html_to_markdown.h"

int main(void) {
    HTMConversionResult* result = htm_convert("<dl><dt>Term One</dt><dd>Definition of term one.</dd><dt>Term Two</dt><dd>Definition of term two.</dd></dl>", NULL);
    htm_conversion_result_free(result);
    return EXIT_SUCCESS;
}

```
