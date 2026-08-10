```c title="C"
#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "html_to_markdown.h"

static int32_t c_visitor_visitor_figure_skip_visit_figure_start(const HTMHtmContext* _ctx, void* _user_data, char** out_custom, size_t* out_len) {
    (void)_ctx;
    (void)_user_data;
    (void)out_custom;
    (void)out_len;
    return 2;
}

int main(void) {

    /* Visitor removes figure elements with their captions */

    HTMHtmVisitorCallbacks _callbacks;
    memset(&_callbacks, 0, sizeof(_callbacks));
    _callbacks.visit_figure_start = c_visitor_visitor_figure_skip_visit_figure_start;

    HTMHtmVisitor* _visitor = htm_visitor_create(&_callbacks);
    assert(_visitor != NULL && "visitor create failed");

    HTMConversionOptions* _options = htm_conversion_options_from_json("{}");
    assert(_options != NULL && "options from_json failed");

    htm_options_set_visitor(_options, _visitor);

    HTMConversionResult* _result = htm_convert("<p>See the chart below:</p><figure><img src=\"chart.svg\"><figcaption>Revenue Trends 2020-2024</figcaption></figure><p>As shown in the chart above.</p>", _options);
    assert(_result != NULL && "visitor call failed");

    char* _json = htm_conversion_result_to_json(_result);
    assert(_json != NULL && "result to_json failed");
    char* _content = alef_json_get_string(_json, "content");

    assert(_content != NULL && strstr(_content, "See the chart below:") != NULL && "expected to contain substring");
    assert(_content != NULL && strstr(_content, "As shown in the chart above.") != NULL && "expected to contain substring");
    assert((_content == NULL || strstr(_content, "Revenue Trends") == NULL) && "expected NOT to contain substring");
    assert((_content == NULL || strstr(_content, "chart.svg") == NULL) && "expected NOT to contain substring");

    free(_content);
    htm_free_string(_json);
    htm_conversion_result_free(_result);
    htm_conversion_options_free(_options);
    htm_visitor_free(_visitor);
    return EXIT_SUCCESS;
}

```
