```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
use HtmlToMarkdown\ConversionOptions;
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["stripTags" => ["div", "span"]]));
$result = HtmlToMarkdown::convert("<div class='wrapper'><p>Inside div</p></div><p>Outside <span class='hl'>span text</span></p>", $options);

```
