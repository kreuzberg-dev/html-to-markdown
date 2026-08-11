---
id: fixture_php_options_newline_backslash
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
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["newlineStyle" => "Backslash"]));
$result = HtmlToMarkdown::convert("<p>Line one<br>Line two</p>", $options);

```
