---
id: fixture_php_options_list_indent_width_four
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
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["listIndentWidth" => 4]));
$result = HtmlToMarkdown::convert("<ul><li>Outer<ul><li>Inner</li></ul></li></ul>", $options);

```
