---
id: fixture_c_visitor_horizontal_rule_skip
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

static int32_t c_visitor_visitor_horizontal_rule_skip_visit_horizontal_rule(const HTMHtmContext* _ctx, void* _user_data, char** out_custom, size_t* out_len) {
    (void)_ctx;
    (void)_user_data;
    (void)out_custom;
    (void)out_len;
    return 2;
}

int main(void) {

    /* Visitor removes all horizontal rules */

    HTMHtmVisitorCallbacks _callbacks;
    memset(&_callbacks, 0, sizeof(_callbacks));
    _callbacks.visit_horizontal_rule = c_visitor_visitor_horizontal_rule_skip_visit_horizontal_rule;

    HTMHtmVisitor* _visitor = htm_visitor_create(&_callbacks);
    assert(_visitor != NULL && "visitor create failed");

    HTMConversionOptions* _options = htm_conversion_options_from_json("{}");
    assert(_options != NULL && "options from_json failed");

    htm_options_set_visitor(_options, _visitor);

    HTMConversionResult* _result = htm_convert("<p>Part 1</p><hr><p>Part 2</p><hr><p>Part 3</p>", _options);
    assert(_result != NULL && "visitor call failed");

    char* _json = htm_conversion_result_to_json(_result);
    assert(_json != NULL && "result to_json failed");
    char* _content = alef_json_get_string(_json, "content");

    assert(_content != NULL && strstr(_content, "Part 1") != NULL && "expected to contain substring");
    assert(_content != NULL && strstr(_content, "Part 2") != NULL && "expected to contain substring");
    assert(_content != NULL && strstr(_content, "Part 3") != NULL && "expected to contain substring");
    assert((_content == NULL || strstr(_content, "---") == NULL) && "expected NOT to contain substring");
    assert((_content == NULL || strstr(_content, "skip") == NULL) && "expected NOT to contain substring");
    assert((_content == NULL || strstr(_content, "Skip") == NULL) && "expected NOT to contain substring");

    free(_content);
    htm_free_string(_json);
    htm_conversion_result_free(_result);
    htm_conversion_options_free(_options);
    htm_visitor_free(_visitor);
    return EXIT_SUCCESS;
}

```
