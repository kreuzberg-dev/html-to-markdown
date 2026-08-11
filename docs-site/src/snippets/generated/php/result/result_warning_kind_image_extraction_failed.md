---
id: fixture_php_result_warning_kind_image_extraction_failed
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
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["extractImages" => true]));
$result = HtmlToMarkdown::convert("<p>Text<img src=\"data:BADMIME\" alt=\"broken\">end</p>", $options);

```
