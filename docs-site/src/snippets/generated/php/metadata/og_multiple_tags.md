```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
$result = HtmlToMarkdown::convert("<html><head><meta property=\"og:title\" content=\"Article Title\"><meta property=\"og:type\" content=\"article\"><meta property=\"og:url\" content=\"https://example.com/article\"><meta property=\"og:site_name\" content=\"Example Site\"><meta property=\"og:description\" content=\"An interesting article.\"><meta property=\"og:image\" content=\"https://example.com/article.jpg\"></head><body><article><p>Article content here.</p></article></body></html>");

```
