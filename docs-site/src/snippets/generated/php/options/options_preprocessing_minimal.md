---
id: fixture_php_options_preprocessing_minimal
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
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["preprocessing" => ["preset" => "Minimal"]]));
$result = HtmlToMarkdown::convert("<nav>Navigation</nav><p>Content</p><footer>Footer</footer>", $options);

```
