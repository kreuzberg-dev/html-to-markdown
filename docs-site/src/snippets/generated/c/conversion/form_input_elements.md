```c title="C"
#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "html_to_markdown.h"

int main(void) {
    HTMConversionOptions* options_handle = htm_conversion_options_from_json("{\"preprocessing\":{\"remove_forms\":false}}");
    HTMConversionResult* result = htm_convert("<form><label for=\"name\">Name:</label><input type=\"text\" id=\"name\" placeholder=\"Enter name\"></form>", options_handle);
    htm_conversion_options_free(options_handle);
    htm_conversion_result_free(result);
    return EXIT_SUCCESS;
}

```
