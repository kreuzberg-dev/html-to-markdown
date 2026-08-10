```c title="C"
#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "html_to_markdown.h"

int main(void) {
    HTMConversionOptions* options_handle = htm_conversion_options_from_json("{\"include_document_structure\":true}");
    HTMConversionResult* result = htm_convert("<h1>Main Title</h1><h2>Section One</h2><p>Section one content.</p><h2>Section Two</h2><p>Section two content.</p>", options_handle);
    htm_conversion_options_free(options_handle);
    htm_conversion_result_free(result);
    return EXIT_SUCCESS;
}

```
