```c title="C"
#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "html_to_markdown.h"

int main(void) {
    HTMConversionOptions* options_handle = htm_conversion_options_from_json("{\"include_document_structure\":true}");
    HTMConversionResult* result = htm_convert("<h1>Top Level</h1><p>Top intro.</p><h2>Mid Level</h2><p>Mid content.</p><h3>Deep Level</h3><p>Deep content.</p>", options_handle);
    htm_conversion_options_free(options_handle);
    htm_conversion_result_free(result);
    return EXIT_SUCCESS;
}

```
