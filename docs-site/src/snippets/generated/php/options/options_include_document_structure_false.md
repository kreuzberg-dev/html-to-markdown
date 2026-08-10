```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
use HtmlToMarkdown\ConversionOptions;
use HtmlToMarkdown\DocumentStructure;
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["includeDocumentStructure" => false]));
$result = HtmlToMarkdown::convert("<article><h1>Heading</h1><p>Paragraph body.</p></article>", $options);

```
