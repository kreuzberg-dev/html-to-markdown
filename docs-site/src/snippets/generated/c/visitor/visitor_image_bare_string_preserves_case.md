```c title="C"
#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "html_to_markdown.h"

static int32_t c_visitor_visitor_image_bare_string_preserves_case_visit_image(const HTMHtmContext* _ctx, void* _user_data, const char* _src, const char* _alt, const char* _title, char** out_custom, size_t* out_len) {
    (void)_ctx;
    (void)_user_data;
    (void)_title;
    char* _buf = (char*)malloc(599);
    if (!_buf) { (void)out_custom; (void)out_len; return 0; }
    snprintf(_buf, 599, "[image: %s -> %s]", (_alt ? _alt : ""), (_src ? _src : ""));
    if (out_custom) *out_custom = _buf;
    if (out_len) *out_len = _buf ? strlen(_buf) : 0;
    return 1;
}

int main(void) {

    /* Visitor returns bare-string image replacement; mixed-case alt/src preserved (regression guard for issue #350) */

    HTMHtmVisitorCallbacks _callbacks;
    memset(&_callbacks, 0, sizeof(_callbacks));
    _callbacks.visit_image = c_visitor_visitor_image_bare_string_preserves_case_visit_image;

    HTMHtmVisitor* _visitor = htm_visitor_create(&_callbacks);
    assert(_visitor != NULL && "visitor create failed");

    HTMConversionOptions* _options = htm_conversion_options_from_json("{}");
    assert(_options != NULL && "options from_json failed");

    htm_options_set_visitor(_options, _visitor);

    HTMConversionResult* _result = htm_convert("<img src=\"PhotoOne.JPG\" alt=\"Sunset Over Bay\">", _options);
    assert(_result != NULL && "visitor call failed");

    char* _json = htm_conversion_result_to_json(_result);
    assert(_json != NULL && "result to_json failed");
    char* _content = alef_json_get_string(_json, "content");

    assert(_content != NULL && strstr(_content, "[image: Sunset Over Bay -> PhotoOne.JPG]") != NULL && "expected to contain substring");
    assert((_content == NULL || strstr(_content, "sunset over bay") == NULL) && "expected NOT to contain substring");
    assert((_content == NULL || strstr(_content, "photoone.jpg") == NULL) && "expected NOT to contain substring");

    free(_content);
    htm_free_string(_json);
    htm_conversion_result_free(_result);
    htm_conversion_options_free(_options);
    htm_visitor_free(_visitor);
    return EXIT_SUCCESS;
}

```
