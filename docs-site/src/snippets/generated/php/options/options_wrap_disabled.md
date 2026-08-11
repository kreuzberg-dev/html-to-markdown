---
id: fixture_php_options_wrap_disabled
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
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["wrap" => false]));
$result = HtmlToMarkdown::convert("<p>This is a long paragraph that should not be wrapped at all because wrapping is disabled.</p>", $options);

```
