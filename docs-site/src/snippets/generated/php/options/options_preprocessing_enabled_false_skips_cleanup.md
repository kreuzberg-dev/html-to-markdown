---
id: fixture_php_options_preprocessing_enabled_false_skips_cleanup
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
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["preprocessing" => ["enabled" => false]]));
$result = HtmlToMarkdown::convert("<nav>NavSection</nav><p>Paragraph</p>", $options);

```
