```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
use HtmlToMarkdown\ConversionOptions;
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["whitespaceMode" => "Normalized"]));
$result = HtmlToMarkdown::convert("<p>Text   with    extra   spaces.</p>", $options);

```
