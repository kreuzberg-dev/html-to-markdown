---
id: fixture_c_twitter_card_tags
language: c
target: c
level: typecheck
requires: []
side_effect: safe
---

```c title="C"
#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "html_to_markdown.h"

int main(void) {
    HTMConversionResult* result = htm_convert("<html><head><meta name=\"twitter:card\" content=\"summary_large_image\"><meta name=\"twitter:site\" content=\"@examplesite\"><meta name=\"twitter:title\" content=\"Twitter Card Title\"><meta name=\"twitter:description\" content=\"Twitter card description.\"><meta name=\"twitter:image\" content=\"https://example.com/twitter-image.jpg\"></head><body><p>Content</p></body></html>", NULL);
    htm_conversion_result_free(result);
    return EXIT_SUCCESS;
}

```
