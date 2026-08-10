```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
use HtmlToMarkdown\ConversionOptions;
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["headingStyle" => "Underlined"]));
$result = HtmlToMarkdown::convert("<h1>Main Title</h1>", $options);

```
