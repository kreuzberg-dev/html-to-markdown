---
id: fixture_php_options_encoding_utf8
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
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["encoding" => "utf-8"]));
$result = HtmlToMarkdown::convert("<p>Café naïve résumé</p>", $options);

```
