---
id: fixture_php_options_highlight_double_equal
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
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["highlightStyle" => "DoubleEqual"]));
$result = HtmlToMarkdown::convert("<p>Text with <mark>highlighted</mark> here.</p>", $options);

```
