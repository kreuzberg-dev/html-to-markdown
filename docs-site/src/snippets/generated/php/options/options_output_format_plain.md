---
id: fixture_php_options_output_format_plain
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
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["outputFormat" => "Plain"]));
$result = HtmlToMarkdown::convert("<h1>Title</h1><p>Some <strong>bold</strong> text.</p>", $options);

```
