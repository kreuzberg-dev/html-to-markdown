```c
#include "html_to_markdown.h"
#include <stdio.h>

int main(void) {
    HTMConversionOptions *options =
        htm_conversion_options_from_json("{\"heading_style\":\"atx\",\"wrap\":true}");
    if (options == NULL) {
        fprintf(stderr, "options failed: %s\n", htm_last_error_context());
        return 1;
    }

    HTMConversionResult *result = htm_convert("<h1>Title</h1><p>Paragraph</p>", options);
    htm_conversion_options_free(options);

    if (result == NULL) {
        fprintf(stderr, "convert failed: %s\n", htm_last_error_context());
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
