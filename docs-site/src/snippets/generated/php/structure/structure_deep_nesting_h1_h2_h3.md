```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
use HtmlToMarkdown\ConversionOptions;
use HtmlToMarkdown\DocumentStructure;
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["includeDocumentStructure" => true]));
$result = HtmlToMarkdown::convert("<h1>Top Level</h1><p>Top intro.</p><h2>Mid Level</h2><p>Mid content.</p><h3>Deep Level</h3><p>Deep content.</p>", $options);

```
