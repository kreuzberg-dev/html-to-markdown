---
id: fixture_php_options_url_escape_style_percent_image
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
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["urlEscapeStyle" => "percent"]));
$result = HtmlToMarkdown::convert("<img src=\"/img (1) <draft>.png\" alt=\"alt\">", $options);

```
