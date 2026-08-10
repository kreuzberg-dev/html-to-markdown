```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
use HtmlToMarkdown\ConversionOptions;
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["convertAsInline" => true]));
$result = HtmlToMarkdown::convert("<p>One</p><p>Two</p>", $options);

```
