```c title="C"
#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "html_to_markdown.h"

int main(void) {
    HTMConversionResult* result = htm_convert("<img src=\"chart.png\" alt=\"Sales chart\" title=\"Q3 Sales\">", NULL);
    htm_conversion_result_free(result);
    return EXIT_SUCCESS;
}

```
