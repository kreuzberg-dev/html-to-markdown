```c title="C"
#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "html_to_markdown.h"

int main(void) {
    HTMConversionResult* result = htm_convert("<html><head><script>alert('xss')</script></head><body><script>document.write('hello')</script></body></html>", NULL);
    htm_conversion_result_free(result);
    return EXIT_SUCCESS;
}

```
