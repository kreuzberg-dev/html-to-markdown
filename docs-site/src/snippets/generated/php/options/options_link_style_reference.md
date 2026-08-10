```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
use HtmlToMarkdown\ConversionOptions;
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["linkStyle" => "Reference"]));
$result = HtmlToMarkdown::convert("<p><a href='https://example.com'>Example</a> and <a href='https://other.com'>Other</a></p>", $options);

```
