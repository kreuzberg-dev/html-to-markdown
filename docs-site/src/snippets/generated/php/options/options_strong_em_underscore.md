```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
use HtmlToMarkdown\ConversionOptions;
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["strongEmSymbol" => "_"]));
$result = HtmlToMarkdown::convert("<p><strong>bold</strong> and <em>italic</em></p>", $options);

```
