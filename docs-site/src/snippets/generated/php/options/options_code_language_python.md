---
id: fixture_php_options_code_language_python
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
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["codeLanguage" => "python"]));
$result = HtmlToMarkdown::convert("<pre><code>def hello(): pass</code></pre>", $options);

```
