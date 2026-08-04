```c
#include "html_to_markdown.h"
#include <stdio.h>

/* Each callback returns an int32 status code:
 *   HTMHTM_VISIT_CONTINUE      — use default conversion
 *   HTMHTM_VISIT_SKIP          — drop the element
 *   HTMHTM_VISIT_PRESERVE_HTML — emit the raw HTML
 *   HTMHTM_VISIT_CUSTOM        — replace with the string written to *out_custom
 *   HTMHTM_VISIT_ERROR         — abort conversion with the error in *out_custom
 */
static int32_t visit_heading(const struct HTMHtmContext *ctx,
                             void *user_data,
                             uint32_t level,
                             const char *text,
                             const char *id,
                             char **out_custom,
                             uintptr_t *out_len) {
    (void)ctx; (void)user_data; (void)level; (void)text; (void)id; (void)out_custom; (void)out_len;
    return HTMHTM_VISIT_CONTINUE;
}

int main(void) {
    HTMHtmVisitorCallbacks callbacks = {0};
    callbacks.visit_heading = visit_heading;

    HTMHtmVisitor *visitor = htm_visitor_create(&callbacks);
    HTMConversionOptions *options = htm_conversion_options_default();
    htm_options_set_visitor(options, visitor);

    HTMConversionResult *result = htm_convert("<h1>Title</h1><p>Content</p>", options);

    htm_conversion_options_free(options);
    htm_visitor_free(visitor);

    if (result == NULL) {
        fprintf(stderr, "convert failed: %s\n", htm_last_error_context());
        return 1;
    }

    char *content = htm_conversion_result_content(result);
    if (content != NULL) {
        printf("%s\n", content);
        htm_free_string(content);
    }
    htm_conversion_result_free(result);
    return 0;
}
```
