```c title="C"
#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "html_to_markdown.h"

int main(void) {
    HTMConversionOptions* options_handle = htm_conversion_options_from_json("{\"extract_images\":true}");
    HTMConversionResult* result = htm_convert("<p>Text<img src=\"data:BADMIME\" alt=\"broken\">end</p>", options_handle);
    htm_conversion_options_free(options_handle);
    htm_conversion_result_free(result);
    return EXIT_SUCCESS;
}

```
