```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
use HtmlToMarkdown\ConversionOptions;
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["headingStyle" => "Atx", "outputFormat" => "Markdown"]));
$result = HtmlToMarkdown::convert("<h1>Title</h1><p>Some text.</p>", $options);

```
