```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
use HtmlToMarkdown\ConversionOptions;
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["urlEscapeStyle" => "percent"]));
$result = HtmlToMarkdown::convert("<img src=\"/img (1) <draft>.png\" alt=\"alt\">", $options);

```
