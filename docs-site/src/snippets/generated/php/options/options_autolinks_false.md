---
id: fixture_php_options_autolinks_false
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
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["autolinks" => false]));
$result = HtmlToMarkdown::convert("<p><a href='https://example.com'>https://example.com</a></p>", $options);

```
