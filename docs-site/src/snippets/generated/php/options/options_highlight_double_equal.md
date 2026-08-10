```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
use HtmlToMarkdown\ConversionOptions;
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["highlightStyle" => "DoubleEqual"]));
$result = HtmlToMarkdown::convert("<p>Text with <mark>highlighted</mark> here.</p>", $options);

```
