```c title="C"
#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "html_to_markdown.h"

static int32_t c_visitor_visitor_deeply_nested_skip_visit_mark(const HTMHtmContext* _ctx, void* _user_data, const char* _text, char** out_custom, size_t* out_len) {
    (void)_ctx;
    (void)_user_data;
    (void)out_custom;
    (void)out_len;
    (void)_text;
    return 2;
}

int main(void) {

    /* Visitor skips deeply nested elements */

    HTMHtmVisitorCallbacks _callbacks;
    memset(&_callbacks, 0, sizeof(_callbacks));
    _callbacks.visit_mark = c_visitor_visitor_deeply_nested_skip_visit_mark;

    HTMHtmVisitor* _visitor = htm_visitor_create(&_callbacks);
    assert(_visitor != NULL && "visitor create failed");

    HTMConversionOptions* _options = htm_conversion_options_from_json("{}");
    assert(_options != NULL && "options from_json failed");

    htm_options_set_visitor(_options, _visitor);

    HTMConversionResult* _result = htm_convert("<div><p>Outer <em>emphasis <strong>with bold <mark>and highlight</mark></strong></em> text</p></div>", _options);
    assert(_result != NULL && "visitor call failed");

    char* _json = htm_conversion_result_to_json(_result);
    assert(_json != NULL && "result to_json failed");
    char* _content = alef_json_get_string(_json, "content");

    assert(_content != NULL && strstr(_content, "Outer") != NULL && "expected to contain substring");
    assert(_content != NULL && strstr(_content, "text") != NULL && "expected to contain substring");
    assert((_content == NULL || strstr(_content, "highlight") == NULL) && "expected NOT to contain substring");

    free(_content);
    htm_free_string(_json);
    htm_conversion_result_free(_result);
    htm_conversion_options_free(_options);
    htm_visitor_free(_visitor);
    return EXIT_SUCCESS;
}

```
