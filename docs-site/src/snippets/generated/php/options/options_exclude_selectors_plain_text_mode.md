---
id: fixture_php_options_exclude_selectors_plain_text_mode
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
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["excludeSelectors" => [".nav"], "outputFormat" => "Plain"]));
$result = HtmlToMarkdown::convert("<body><div class=\"nav\">Navigation</div><p>Article body</p></body>", $options);

```
