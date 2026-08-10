```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
use HtmlToMarkdown\ConversionOptions;
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["newlineStyle" => "Backslash"]));
$result = HtmlToMarkdown::convert("<p>Line one<br>Line two</p>", $options);

```
