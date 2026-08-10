```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
use HtmlToMarkdown\ConversionOptions;
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["extractImages" => true]));
$result = HtmlToMarkdown::convert("<p>Text<img src=\"data:BADMIME\" alt=\"broken\">end</p>", $options);

```
