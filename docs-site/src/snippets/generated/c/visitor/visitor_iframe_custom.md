---
id: fixture_c_visitor_iframe_custom
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

static int32_t c_visitor_visitor_iframe_custom_visit_iframe(const HTMHtmContext* _ctx, void* _user_data, const char* _src, char** out_custom, size_t* out_len) {
    (void)_ctx;
    (void)_user_data;
    (void)_src;
    char* _buf = strdup("[EMBEDDED: https://maps.example.com/embed]");
    if (out_custom) *out_custom = _buf;
    if (out_len) *out_len = _buf ? strlen(_buf) : 0;
    return 1;
}

int main(void) {

    /* Visitor replaces embedded iframe with custom text */

    HTMHtmVisitorCallbacks _callbacks;
    memset(&_callbacks, 0, sizeof(_callbacks));
    _callbacks.visit_iframe = c_visitor_visitor_iframe_custom_visit_iframe;

    HTMHtmVisitor* _visitor = htm_visitor_create(&_callbacks);
    assert(_visitor != NULL && "visitor create failed");

    HTMConversionOptions* _options = htm_conversion_options_from_json("{}");
    assert(_options != NULL && "options from_json failed");

    htm_options_set_visitor(_options, _visitor);

    HTMConversionResult* _result = htm_convert("<p>Embedded map:</p><iframe src=\"https://maps.example.com/embed\" width=\"400\" height=\"300\"></iframe><p>End of map</p>", _options);
    assert(_result != NULL && "visitor call failed");

    char* _json = htm_conversion_result_to_json(_result);
    assert(_json != NULL && "result to_json failed");
    char* _content = alef_json_get_string(_json, "content");

    assert(_content != NULL && strstr(_content, "[EMBEDDED: https://maps.example.com/embed]") != NULL && "expected to contain substring");
    assert(_content != NULL && strstr(_content, "Embedded map:") != NULL && "expected to contain substring");
    assert(_content != NULL && strstr(_content, "End of map") != NULL && "expected to contain substring");

    free(_content);
    htm_free_string(_json);
    htm_conversion_result_free(_result);
    htm_conversion_options_free(_options);
    htm_visitor_free(_visitor);
    return EXIT_SUCCESS;
}

```
