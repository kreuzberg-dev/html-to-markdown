```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
use HtmlToMarkdown\ConversionOptions;
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["highlightStyle" => "None"]));
$result = HtmlToMarkdown::convert("<p>Text with <mark>plain</mark> content.</p>", $options);

```
