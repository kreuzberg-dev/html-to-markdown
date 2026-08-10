```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
use HtmlToMarkdown\ConversionOptions;
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["listIndentType" => "Tabs"]));
$result = HtmlToMarkdown::convert("<ul><li>Parent<ul><li>Child</li></ul></li></ul>", $options);

```
