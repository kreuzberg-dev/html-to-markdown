```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
use HtmlToMarkdown\ConversionOptions;
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["codeBlockStyle" => "Tildes"]));
$result = HtmlToMarkdown::convert("<pre><code>some code</code></pre>", $options);

```
