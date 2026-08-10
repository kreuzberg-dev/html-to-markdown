```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
use HtmlToMarkdown\ConversionOptions;
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["supSymbol" => "^"]));
$result = HtmlToMarkdown::convert("<p>x<sup>2</sup></p>", $options);

```
