---
id: fixture_php_options_whitespace_strict
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
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["whitespaceMode" => "Strict"]));
$result = HtmlToMarkdown::convert("<p>Preserved   spacing.</p>", $options);

```
