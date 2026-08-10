```c title="C"
#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "html_to_markdown.h"

int main(void) {
    HTMConversionResult* result = htm_convert("<html><head><title>Shop Page</title><script type=\"application/ld+json\">{\"@context\":\"https://schema.org\",\"@type\":\"Product\",\"name\":\"Widget\",\"price\":\"9.99\"}</script><script type=\"application/ld+json\">{\"@context\":\"https://schema.org\",\"@type\":\"BreadcrumbList\",\"itemListElement\":[{\"@type\":\"ListItem\",\"position\":1,\"name\":\"Home\"}]}</script></head><body><h1>Widget</h1><p>A great widget for all purposes.</p></body></html>", NULL);
    htm_conversion_result_free(result);
    return EXIT_SUCCESS;
}

```
