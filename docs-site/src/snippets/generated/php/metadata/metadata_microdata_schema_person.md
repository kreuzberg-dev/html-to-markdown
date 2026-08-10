```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
use HtmlToMarkdown\ConversionOptions;
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["extractMetadata" => true]));
$result = HtmlToMarkdown::convert("<html><head><title>Contact</title></head><body><div itemscope itemtype=\"https://schema.org/Person\"><span itemprop=\"name\">John Smith</span><span itemprop=\"email\">john@example.com</span><span itemprop=\"telephone\">+1-555-0100</span></div></body></html>", $options);

```
