---
id: fixture_c_visitor_definition_list_custom_format
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

static int32_t c_visitor_visitor_definition_list_custom_format_visit_definition_description(const HTMHtmContext* _ctx, void* _user_data, const char* _text, char** out_custom, size_t* out_len) {
    (void)_ctx;
    (void)_user_data;
    char* _buf = (char*)malloc(328);
    if (!_buf) { (void)out_custom; (void)out_len; return 0; }
    snprintf(_buf, 328, "> %s", (_text ? _text : ""));
    if (out_custom) *out_custom = _buf;
    if (out_len) *out_len = _buf ? strlen(_buf) : 0;
    return 1;
}

static int32_t c_visitor_visitor_definition_list_custom_format_visit_definition_term(const HTMHtmContext* _ctx, void* _user_data, const char* _text, char** out_custom, size_t* out_len) {
    (void)_ctx;
    (void)_user_data;
    char* _buf = (char*)malloc(330);
    if (!_buf) { (void)out_custom; (void)out_len; return 0; }
    snprintf(_buf, 330, "### %s", (_text ? _text : ""));
    if (out_custom) *out_custom = _buf;
    if (out_len) *out_len = _buf ? strlen(_buf) : 0;
    return 1;
}

int main(void) {

    /* Visitor formats definition lists with custom templates */

    HTMHtmVisitorCallbacks _callbacks;
    memset(&_callbacks, 0, sizeof(_callbacks));
    _callbacks.visit_definition_description = c_visitor_visitor_definition_list_custom_format_visit_definition_description;
    _callbacks.visit_definition_term = c_visitor_visitor_definition_list_custom_format_visit_definition_term;

    HTMHtmVisitor* _visitor = htm_visitor_create(&_callbacks);
    assert(_visitor != NULL && "visitor create failed");

    HTMConversionOptions* _options = htm_conversion_options_from_json("{}");
    assert(_options != NULL && "options from_json failed");

    htm_options_set_visitor(_options, _visitor);

    HTMConversionResult* _result = htm_convert("<dl><dt>Python</dt><dd>A high-level programming language</dd><dt>JavaScript</dt><dd>A scripting language for web browsers</dd></dl>", _options);
    assert(_result != NULL && "visitor call failed");

    char* _json = htm_conversion_result_to_json(_result);
    assert(_json != NULL && "result to_json failed");
    char* _content = alef_json_get_string(_json, "content");

    assert(_content != NULL && strstr(_content, "### Python") != NULL && "expected to contain substring");
    assert(_content != NULL && strstr(_content, "### JavaScript") != NULL && "expected to contain substring");
    assert(_content != NULL && strstr(_content, "> A high-level programming language") != NULL && "expected to contain substring");
    assert(_content != NULL && strstr(_content, "> A scripting language for web browsers") != NULL && "expected to contain substring");

    free(_content);
    htm_free_string(_json);
    htm_conversion_result_free(_result);
    htm_conversion_options_free(_options);
    htm_visitor_free(_visitor);
    return EXIT_SUCCESS;
}

```
