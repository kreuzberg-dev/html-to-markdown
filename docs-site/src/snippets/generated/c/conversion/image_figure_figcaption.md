```c title="C"
#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "html_to_markdown.h"

int main(void) {
    HTMConversionResult* result = htm_convert("<figure><img src=\"sunset.jpg\" alt=\"A sunset\"><figcaption>Beautiful sunset over the ocean</figcaption></figure>", NULL);
    htm_conversion_result_free(result);
    return EXIT_SUCCESS;
}

```
