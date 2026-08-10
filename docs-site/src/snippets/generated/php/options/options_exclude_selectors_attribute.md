```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
use HtmlToMarkdown\ConversionOptions;
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["excludeSelectors" => ["[role='complementary']"]]));
$result = HtmlToMarkdown::convert("<body><div role=\"complementary\">Sidebar</div><p>Primary text</p></body>", $options);

```
