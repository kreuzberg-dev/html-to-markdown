---
id: fixture_php_options_sub_symbol_tilde
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
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["subSymbol" => "~"]));
$result = HtmlToMarkdown::convert("<p>H<sub>2</sub>O</p>", $options);

```
