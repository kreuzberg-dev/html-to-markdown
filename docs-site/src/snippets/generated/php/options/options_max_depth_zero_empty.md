---
id: fixture_php_options_max_depth_zero_empty
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
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["maxDepth" => 0]));
$result = HtmlToMarkdown::convert("<p>Hello</p>", $options);

```
