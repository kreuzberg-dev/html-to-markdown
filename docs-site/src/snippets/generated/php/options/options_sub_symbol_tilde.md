```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
use HtmlToMarkdown\ConversionOptions;
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["subSymbol" => "~"]));
$result = HtmlToMarkdown::convert("<p>H<sub>2</sub>O</p>", $options);

```
