---
id: fixture_c_encoding_cjk_characters
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
    HTMConversionResult* result = htm_convert("<p>中文内容</p><p>日本語テキスト</p><p>한국어 텍스트</p>", NULL);
    htm_conversion_result_free(result);
    return EXIT_SUCCESS;
}

```
