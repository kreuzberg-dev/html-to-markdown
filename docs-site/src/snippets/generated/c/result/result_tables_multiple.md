```c title="C"
#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "html_to_markdown.h"

int main(void) {
    HTMConversionOptions* options_handle = htm_conversion_options_from_json("{\"include_document_structure\":true}");
    HTMConversionResult* result = htm_convert("<table><tr><th>A</th></tr><tr><td>1</td></tr></table><p>Between</p><table><tr><th>B</th></tr><tr><td>2</td></tr></table>", options_handle);
    htm_conversion_options_free(options_handle);
    htm_conversion_result_free(result);
    return EXIT_SUCCESS;
}

```
