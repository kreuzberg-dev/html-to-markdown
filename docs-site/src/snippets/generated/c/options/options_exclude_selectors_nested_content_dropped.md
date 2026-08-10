```c title="C"
#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "html_to_markdown.h"

int main(void) {
    HTMConversionOptions* options_handle = htm_conversion_options_from_json("{\"exclude_selectors\":[\".sidebar\"]}");
    HTMConversionResult* result = htm_convert("<body><aside class=\"sidebar\"><h2>Related</h2><p>Sidebar text</p></aside><main><p>Main text</p></main></body>", options_handle);
    htm_conversion_options_free(options_handle);
    htm_conversion_result_free(result);
    return EXIT_SUCCESS;
}

```
