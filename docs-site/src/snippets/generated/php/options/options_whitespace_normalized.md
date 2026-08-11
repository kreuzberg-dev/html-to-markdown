---
id: fixture_php_options_whitespace_normalized
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
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["whitespaceMode" => "Normalized"]));
$result = HtmlToMarkdown::convert("<p>Text   with    extra   spaces.</p>", $options);

```
