```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
use HtmlToMarkdown\ConversionOptions;
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["extractMetadata" => true]));
$result = HtmlToMarkdown::convert("<html><head><title>Page</title><link rel=\"canonical\" href=\"https://example.com/canonical-page\"></head><body><p>Content</p></body></html>", $options);

```
