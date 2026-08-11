---
id: fixture_php_options_convert_as_inline
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
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["convertAsInline" => true]));
$result = HtmlToMarkdown::convert("<p>One</p><p>Two</p>", $options);

```
