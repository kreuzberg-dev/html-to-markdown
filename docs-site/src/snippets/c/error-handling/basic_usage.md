```c
#include "html_to_markdown.h"
#include <stdio.h>

int main(void) {
    /* Binary data (detected via magic bytes) is rejected before parsing. */
    HTMConversionResult *result = htm_convert("%PDF-1.4 not actually HTML", NULL);
    if (result == NULL) {
        fprintf(stderr, "convert failed (code %d): %s\n",
                htm_last_error_code(), htm_last_error_context());
        return 1;
    }

    char *content = htm_conversion_result_content(result);
    if (content != NULL) {
        printf("%s\n", content);
        htm_free_string(content);
    }

    htm_conversion_result_free(result);
    return 0;
}
```
