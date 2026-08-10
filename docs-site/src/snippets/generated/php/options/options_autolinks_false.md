```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
use HtmlToMarkdown\ConversionOptions;
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["autolinks" => false]));
$result = HtmlToMarkdown::convert("<p><a href='https://example.com'>https://example.com</a></p>", $options);

```
