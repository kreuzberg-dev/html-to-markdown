```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
use HtmlToMarkdown\ConversionOptions;
use HtmlToMarkdown\DocumentStructure;
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["includeDocumentStructure" => true]));
$result = HtmlToMarkdown::convert("<h1>Main Title</h1><h2>Section One</h2><p>Section one content.</p><h2>Section Two</h2><p>Section two content.</p>", $options);

```
