```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
use HtmlToMarkdown\ConversionOptions;
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["defaultTitle" => true]));
$result = HtmlToMarkdown::convert("<p><a href='https://example.com'>Link</a></p>", $options);

```
