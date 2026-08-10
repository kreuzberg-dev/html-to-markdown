```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
use HtmlToMarkdown\ConversionOptions;
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["excludeSelectors" => [".nav"], "outputFormat" => "Plain"]));
$result = HtmlToMarkdown::convert("<body><div class=\"nav\">Navigation</div><p>Article body</p></body>", $options);

```
