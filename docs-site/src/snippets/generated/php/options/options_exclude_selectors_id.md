```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
use HtmlToMarkdown\ConversionOptions;
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["excludeSelectors" => ["#ad-container"]]));
$result = HtmlToMarkdown::convert("<body><div id=\"ad-container\">Buy stuff</div><p>Article text</p></body>", $options);

```
