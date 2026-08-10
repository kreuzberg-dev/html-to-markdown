```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
use HtmlToMarkdown\ConversionOptions;
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["preprocessing" => ["enabled" => false]]));
$result = HtmlToMarkdown::convert("<nav>NavSection</nav><p>Paragraph</p>", $options);

```
