```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
$result = HtmlToMarkdown::convert("<html><head><style>body { color: red; }</style></head><body><style>.foo { margin: 0; }</style></body></html>");

```
