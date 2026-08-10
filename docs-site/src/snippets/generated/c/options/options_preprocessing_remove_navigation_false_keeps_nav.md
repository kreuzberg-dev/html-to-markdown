```c title="C"
#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "html_to_markdown.h"

int main(void) {
    HTMConversionOptions* options_handle = htm_conversion_options_from_json("{\"preprocessing\":{\"remove_navigation\":false}}");
    HTMConversionResult* result = htm_convert("<nav>SiteMenu</nav><main><p>MainContent</p></main><aside>SidebarText</aside>", options_handle);
    htm_conversion_options_free(options_handle);
    htm_conversion_result_free(result);
    return EXIT_SUCCESS;
}

```
