---
id: fixture_php_options_newline_spaces
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
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["newlineStyle" => "Spaces"]));
$result = HtmlToMarkdown::convert("<p>First<br>Second</p>", $options);

```
