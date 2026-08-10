```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
use HtmlToMarkdown\ConversionOptions;
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["excludeSelectors" => [".wrapper"]]));
$result = HtmlToMarkdown::convert("<body><div class=\"wrapper\"><p>Inner paragraph</p></div><p>Outer text</p></body>", $options);

```
