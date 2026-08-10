```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
use HtmlToMarkdown\ConversionOptions;
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["escapeUnderscores" => true]));
$result = HtmlToMarkdown::convert("<p>The variable_name is defined.</p>", $options);

```
