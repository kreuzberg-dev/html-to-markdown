```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
use HtmlToMarkdown\ConversionOptions;
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["outputFormat" => "Plain"]));
$result = HtmlToMarkdown::convert("<h1>Title</h1><p>Some <strong>bold</strong> text.</p>", $options);

```
