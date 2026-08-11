---
id: fixture_php_options_wrap_enabled
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
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["wrap" => true, "wrapWidth" => 40]));
$result = HtmlToMarkdown::convert("<p>This is a long paragraph that should be wrapped at the specified column width when the wrap option is enabled.</p>", $options);

```
