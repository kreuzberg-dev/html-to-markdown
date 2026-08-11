---
id: fixture_php_options_exclude_selectors_empty_noop
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
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["excludeSelectors" => []]));
$result = HtmlToMarkdown::convert("<p>Hello world</p>", $options);

```
