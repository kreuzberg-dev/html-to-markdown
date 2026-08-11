---
id: fixture_php_options_exclude_selectors_vs_strip_tags
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
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["excludeSelectors" => [".wrapper"]]));
$result = HtmlToMarkdown::convert("<body><div class=\"wrapper\"><p>Inner paragraph</p></div><p>Outer text</p></body>", $options);

```
