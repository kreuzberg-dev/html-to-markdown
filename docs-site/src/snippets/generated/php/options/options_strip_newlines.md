---
id: fixture_php_options_strip_newlines
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
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["stripNewlines" => true]));
$result = HtmlToMarkdown::convert("<p>First paragraph.</p><p>Second paragraph.</p>", $options);

```
