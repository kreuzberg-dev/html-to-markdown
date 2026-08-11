---
id: fixture_php_options_heading_style_atx
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
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["headingStyle" => "Atx"]));
$result = HtmlToMarkdown::convert("<h1>Title</h1><h2>Subtitle</h2>", $options);

```
