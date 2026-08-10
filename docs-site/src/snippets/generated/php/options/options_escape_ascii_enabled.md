```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
use HtmlToMarkdown\ConversionOptions;
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["escapeAscii" => true]));
$result = HtmlToMarkdown::convert("<p>Text with # hash and [brackets] and * star</p>", $options);

```
