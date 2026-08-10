```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
use HtmlToMarkdown\ConversionOptions;
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["extractMetadata" => true]));
$result = HtmlToMarkdown::convert("<html lang=\"es\"><head><title>Spanish Page</title></head><body><h1>Hola Mundo</h1><p>Este es un documento en español.</p></body></html>", $options);

```
