```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
use HtmlToMarkdown\ConversionOptions;
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["highlightStyle" => "Bold"]));
$result = HtmlToMarkdown::convert("<p>Text with <mark>highlighted</mark> text.</p>", $options);

```
