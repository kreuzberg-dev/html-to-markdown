```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
use HtmlToMarkdown\ConversionOptions;
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["extractMetadata" => true]));
$result = HtmlToMarkdown::convert("<html><head><title>My Page</title></head><body><p>Content</p></body></html>", $options);

```
