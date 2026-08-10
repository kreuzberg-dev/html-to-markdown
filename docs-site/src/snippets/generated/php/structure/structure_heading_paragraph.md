```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
use HtmlToMarkdown\ConversionOptions;
use HtmlToMarkdown\DocumentStructure;
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["includeDocumentStructure" => true]));
$result = HtmlToMarkdown::convert("<h1>Title</h1><p>A paragraph of text.</p>", $options);

```
