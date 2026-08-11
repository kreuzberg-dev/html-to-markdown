---
id: fixture_php_options_url_escape_style_angle_default
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
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["urlEscapeStyle" => "angle"]));
$result = HtmlToMarkdown::convert("<a href=\"/file (1).pdf\">file</a>", $options);

```
