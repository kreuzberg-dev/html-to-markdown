---
id: fixture_php_options_output_format_djot
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
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["outputFormat" => "Djot"]));
$result = HtmlToMarkdown::convert("<p>Simple paragraph.</p>", $options);

```
