```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
use HtmlToMarkdown\ConversionOptions;
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["escapeAsterisks" => true]));
$result = HtmlToMarkdown::convert("<p>Use 2*3 = 6 in math.</p>", $options);

```
