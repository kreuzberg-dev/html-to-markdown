---
id: fixture_php_options_sup_symbol_caret
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
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["supSymbol" => "^"]));
$result = HtmlToMarkdown::convert("<p>x<sup>2</sup></p>", $options);

```
