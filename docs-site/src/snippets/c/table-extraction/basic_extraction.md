```c
#include "html_to_markdown.h"
#include <stdio.h>

int main(void) {
    const char *html =
        "<table>"
        "<tr><th>Name</th><th>Age</th></tr>"
        "<tr><td>Alice</td><td>30</td></tr>"
        "<tr><td>Bob</td><td>25</td></tr>"
        "</table>";

    /* include_document_structure must be enabled to populate result.tables;
     * with the default options the "tables" array in the JSON is empty. */
    HTMConversionOptions *options =
        htm_conversion_options_from_json("{\"include_document_structure\":true}");
    if (options == NULL) {
        fprintf(stderr, "options failed: %s\n", htm_last_error_context());
        return 1;
    }

    HTMConversionResult *result = htm_convert(html, options);
    htm_conversion_options_free(options);
    if (result == NULL) {
        fprintf(stderr, "convert failed: %s\n", htm_last_error_context());
        return 1;
    }

    char *json = htm_conversion_result_to_json(result);
    if (json != NULL) {
        printf("%s\n", json);  /* contains a populated "tables" array */
        htm_free_string(json);
    }

    htm_conversion_result_free(result);
    return 0;
}
```
