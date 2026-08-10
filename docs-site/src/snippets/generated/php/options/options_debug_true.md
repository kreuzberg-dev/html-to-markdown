```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
use HtmlToMarkdown\ConversionOptions;
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["debug" => true]));
$result = HtmlToMarkdown::convert("<p>Debug test</p>", $options);

```
