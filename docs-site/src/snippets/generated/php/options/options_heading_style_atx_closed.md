---
id: fixture_php_options_heading_style_atx_closed
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
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["headingStyle" => "AtxClosed"]));
$result = HtmlToMarkdown::convert("<h1>Closed Heading</h1>", $options);

```
