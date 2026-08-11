---
id: fixture_php_structure_code_block
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
use HtmlToMarkdown\ConversionOptions;
use HtmlToMarkdown\DocumentStructure;
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["includeDocumentStructure" => true]));
$result = HtmlToMarkdown::convert("<p>Example code:</p><pre><code class=\"language-rust\">fn main() { println!(\"Hello\"); }</code></pre>", $options);

```
