```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
use HtmlToMarkdown\ConversionOptions;
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["wrap" => false]));
$result = HtmlToMarkdown::convert("<p>This is a long paragraph that should not be wrapped at all because wrapping is disabled.</p>", $options);

```
