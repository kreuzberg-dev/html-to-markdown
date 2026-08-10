```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
use HtmlToMarkdown\ConversionOptions;
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["keepInlineImagesIn" => ["p"]]));
$result = HtmlToMarkdown::convert("<p>Text <img src='icon.png' alt='icon'> more text</p>", $options);

```
