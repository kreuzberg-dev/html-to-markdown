```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
use HtmlToMarkdown\ConversionOptions;
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["maxDepth" => 0]));
$result = HtmlToMarkdown::convert("<p>Hello</p>", $options);

```
