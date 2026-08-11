---
id: fixture_php_options_exclude_selectors_class
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
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["excludeSelectors" => [".cookie-banner"]]));
$result = HtmlToMarkdown::convert("<body><div class=\"cookie-banner\">Accept cookies</div><p>Main content</p></body>", $options);

```
