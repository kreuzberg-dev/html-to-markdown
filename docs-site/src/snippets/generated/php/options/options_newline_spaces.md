```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
use HtmlToMarkdown\ConversionOptions;
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["newlineStyle" => "Spaces"]));
$result = HtmlToMarkdown::convert("<p>First<br>Second</p>", $options);

```
