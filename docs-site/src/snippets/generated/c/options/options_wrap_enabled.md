```c title="C"
#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "html_to_markdown.h"

int main(void) {
    HTMConversionOptions* options_handle = htm_conversion_options_from_json("{\"wrap\":true,\"wrap_width\":40}");
    HTMConversionResult* result = htm_convert("<p>This is a long paragraph that should be wrapped at the specified column width when the wrap option is enabled.</p>", options_handle);
    htm_conversion_options_free(options_handle);
    htm_conversion_result_free(result);
    return EXIT_SUCCESS;
}

```
