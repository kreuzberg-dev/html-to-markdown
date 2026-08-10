```c title="C"
#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "html_to_markdown.h"

int main(void) {
    HTMConversionOptions* options_handle = htm_conversion_options_from_json("{\"extract_metadata\":true}");
    HTMConversionResult* result = htm_convert("<html><head><title>Product</title></head><body><div itemscope itemtype=\"https://schema.org/Product\"><h1 itemprop=\"name\">Awesome Widget</h1><span itemprop=\"description\">The best widget on the market</span><span itemprop=\"price\">29.99</span><span itemprop=\"priceCurrency\">USD</span><img itemprop=\"image\" src=\"widget.jpg\" alt=\"Widget\"><span itemprop=\"ratingValue\">4.5</span></div></body></html>", options_handle);
    htm_conversion_options_free(options_handle);
    htm_conversion_result_free(result);
    return EXIT_SUCCESS;
}

```
