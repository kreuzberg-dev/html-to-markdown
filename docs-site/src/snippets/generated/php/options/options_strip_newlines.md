```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
use HtmlToMarkdown\ConversionOptions;
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["stripNewlines" => true]));
$result = HtmlToMarkdown::convert("<p>First paragraph.</p><p>Second paragraph.</p>", $options);

```
