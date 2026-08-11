---
id: fixture_c_visitor_input_skip
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

static int32_t c_visitor_visitor_input_skip_visit_input(const HTMHtmContext* _ctx, void* _user_data, const char* _input_type, const char* _name, const char* _value, char** out_custom, size_t* out_len) {
    (void)_ctx;
    (void)_user_data;
    (void)out_custom;
    (void)out_len;
    (void)_input_type;
    (void)_name;
    (void)_value;
    return 2;
}

int main(void) {

    /* Visitor skips all input elements */

    HTMHtmVisitorCallbacks _callbacks;
    memset(&_callbacks, 0, sizeof(_callbacks));
    _callbacks.visit_input = c_visitor_visitor_input_skip_visit_input;

    HTMHtmVisitor* _visitor = htm_visitor_create(&_callbacks);
    assert(_visitor != NULL && "visitor create failed");

    HTMConversionOptions* _options = htm_conversion_options_from_json("{}");
    assert(_options != NULL && "options from_json failed");

    htm_options_set_visitor(_options, _visitor);

    HTMConversionResult* _result = htm_convert("<p>Sign up:</p><input type=\"text\" name=\"email\" placeholder=\"your@email.com\"><input type=\"checkbox\" name=\"agree\"><p>Continue</p>", _options);
    assert(_result != NULL && "visitor call failed");

    char* _json = htm_conversion_result_to_json(_result);
    assert(_json != NULL && "result to_json failed");
    char* _content = alef_json_get_string(_json, "content");

    assert(_content != NULL && strstr(_content, "Sign up:") != NULL && "expected to contain substring");
    assert(_content != NULL && strstr(_content, "Continue") != NULL && "expected to contain substring");
    assert((_content == NULL || strstr(_content, "email") == NULL) && "expected NOT to contain substring");
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
