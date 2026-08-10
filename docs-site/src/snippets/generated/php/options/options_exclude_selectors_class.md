```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
use HtmlToMarkdown\ConversionOptions;
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["excludeSelectors" => [".cookie-banner"]]));
$result = HtmlToMarkdown::convert("<body><div class=\"cookie-banner\">Accept cookies</div><p>Main content</p></body>", $options);

```
