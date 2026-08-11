---
id: fixture_php_options_list_custom_bullets
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
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["bullets" => "*"]));
$result = HtmlToMarkdown::convert("<ul><li>Item A</li><li>Item B</li></ul>", $options);

```
