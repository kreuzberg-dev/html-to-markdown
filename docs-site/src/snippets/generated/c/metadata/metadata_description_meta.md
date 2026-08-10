```c title="C"
#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "html_to_markdown.h"

int main(void) {
    HTMConversionOptions* options_handle = htm_conversion_options_from_json("{\"extract_metadata\":true}");
    HTMConversionResult* result = htm_convert("<html><head><title>Page</title><meta name=\"description\" content=\"This is the page description.\"></head><body><p>Content</p></body></html>", options_handle);
    htm_conversion_options_free(options_handle);
    htm_conversion_result_free(result);
    return EXIT_SUCCESS;
}

```
