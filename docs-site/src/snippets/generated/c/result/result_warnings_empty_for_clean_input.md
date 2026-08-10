```c title="C"
#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "html_to_markdown.h"

int main(void) {
    HTMConversionResult* result = htm_convert("<h1>Title</h1><p>Clean content with <a href='https://example.com'>a link</a>.</p>", NULL);
    htm_conversion_result_free(result);
    return EXIT_SUCCESS;
}

```
