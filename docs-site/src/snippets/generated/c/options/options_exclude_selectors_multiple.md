```c title="C"
#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "html_to_markdown.h"

int main(void) {
    HTMConversionOptions* options_handle = htm_conversion_options_from_json("{\"exclude_selectors\":[\".nav\",\"footer\"]}");
    HTMConversionResult* result = htm_convert("<body><nav class=\"nav\">Menu</nav><p>Content</p><footer>Footer</footer></body>", options_handle);
    htm_conversion_options_free(options_handle);
    htm_conversion_result_free(result);
    return EXIT_SUCCESS;
}

```
