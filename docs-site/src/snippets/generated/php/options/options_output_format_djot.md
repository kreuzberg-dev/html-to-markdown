```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
use HtmlToMarkdown\ConversionOptions;
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["outputFormat" => "Djot"]));
$result = HtmlToMarkdown::convert("<p>Simple paragraph.</p>", $options);

```
