---
id: fixture_c_visitor_button_custom
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

static int32_t c_visitor_visitor_button_custom_visit_button(const HTMHtmContext* _ctx, void* _user_data, const char* _text, char** out_custom, size_t* out_len) {
    (void)_ctx;
    (void)_user_data;
    char* _buf = (char*)malloc(332);
    if (!_buf) { (void)out_custom; (void)out_len; return 0; }
    snprintf(_buf, 332, "[BTN:%s]", (_text ? _text : ""));
    if (out_custom) *out_custom = _buf;
    if (out_len) *out_len = _buf ? strlen(_buf) : 0;
    return 1;
}

int main(void) {

    /* Visitor replaces button with bracketed text */

    HTMHtmVisitorCallbacks _callbacks;
    memset(&_callbacks, 0, sizeof(_callbacks));
    _callbacks.visit_button = c_visitor_visitor_button_custom_visit_button;

    HTMHtmVisitor* _visitor = htm_visitor_create(&_callbacks);
    assert(_visitor != NULL && "visitor create failed");

    HTMConversionOptions* _options = htm_conversion_options_from_json("{}");
    assert(_options != NULL && "options from_json failed");

    htm_options_set_visitor(_options, _visitor);

    HTMConversionResult* _result = htm_convert("<p>Confirm action: <button type=\"submit\">Click me</button> or <button type=\"reset\">Cancel</button></p>", _options);
    assert(_result != NULL && "visitor call failed");

    char* _json = htm_conversion_result_to_json(_result);
    assert(_json != NULL && "result to_json failed");
    char* _content = alef_json_get_string(_json, "content");

    assert(_content != NULL && strstr(_content, "[BTN:Click me]") != NULL && "expected to contain substring");
    assert(_content != NULL && strstr(_content, "[BTN:Cancel]") != NULL && "expected to contain substring");
    assert(_content != NULL && strstr(_content, "Confirm action:") != NULL && "expected to contain substring");

    free(_content);
    htm_free_string(_json);
    htm_conversion_result_free(_result);
    htm_conversion_options_free(_options);
    htm_visitor_free(_visitor);
    return EXIT_SUCCESS;
}

```
