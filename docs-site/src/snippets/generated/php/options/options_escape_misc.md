```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
use HtmlToMarkdown\ConversionOptions;
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["escapeMisc" => true]));
$result = HtmlToMarkdown::convert("<p>Use # and | and ~ in text.</p>", $options);

```
