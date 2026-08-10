```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
use HtmlToMarkdown\ConversionOptions;
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["extractMetadata" => true]));
$result = HtmlToMarkdown::convert("<html><head><title>Gallery</title></head><body><img src=\"https://example.com/photo1.jpg\" alt=\"Photo 1\"><img src=\"https://example.com/photo2.png\" alt=\"Photo 2\"><img src=\"/local/image.webp\" alt=\"Local image\"></body></html>", $options);

```
