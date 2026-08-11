---
id: fixture_php_options_debug_true
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
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["debug" => true]));
$result = HtmlToMarkdown::convert("<p>Debug test</p>", $options);

```
