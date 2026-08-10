```c title="C"
#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "html_to_markdown.h"

static int32_t c_visitor_visitor_unknown_tag_preservation_visit_custom_element(const HTMHtmContext* _ctx, void* _user_data, const char* _tag_name, const char* _html, char** out_custom, size_t* out_len) {
    (void)_ctx;
    (void)_user_data;
    (void)out_custom;
    (void)out_len;
    (void)_tag_name;
    (void)_html;
    return 3;
}

int main(void) {

    /* Visitor preserves unknown HTML tags as raw HTML */

    HTMHtmVisitorCallbacks _callbacks;
    memset(&_callbacks, 0, sizeof(_callbacks));
    _callbacks.visit_custom_element = c_visitor_visitor_unknown_tag_preservation_visit_custom_element;

    HTMHtmVisitor* _visitor = htm_visitor_create(&_callbacks);
    assert(_visitor != NULL && "visitor create failed");

    HTMConversionOptions* _options = htm_conversion_options_from_json("{}");
    assert(_options != NULL && "options from_json failed");

    htm_options_set_visitor(_options, _visitor);

    HTMConversionResult* _result = htm_convert("<article><p>Article text</p><x-custom>Custom element with content</x-custom><p>More article text</p></article>", _options);
    assert(_result != NULL && "visitor call failed");

    char* _json = htm_conversion_result_to_json(_result);
    assert(_json != NULL && "result to_json failed");
    char* _content = alef_json_get_string(_json, "content");

    assert(_content != NULL && strstr(_content, "Article text") != NULL && "expected to contain substring");
    assert(_content != NULL && strstr(_content, "More article text") != NULL && "expected to contain substring");
    assert(_content != NULL && strstr(_content, "<x-custom>") != NULL && "expected to contain substring");

    free(_content);
    htm_free_string(_json);
    htm_conversion_result_free(_result);
    htm_conversion_options_free(_options);
    htm_visitor_free(_visitor);
    return EXIT_SUCCESS;
}

```
