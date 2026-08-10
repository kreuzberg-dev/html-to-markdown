```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
$result = HtmlToMarkdown::convert("<html><head><title>Fallback Title</title><meta property=\"og:title\" content=\"OG Title\"><meta property=\"og:description\" content=\"OG description text.\"><meta property=\"og:image\" content=\"https://example.com/image.jpg\"></head><body><p>Content</p></body></html>");

```
