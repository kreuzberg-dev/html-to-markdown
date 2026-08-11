---
id: fixture_php_options_default_title_true
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
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["defaultTitle" => true]));
$result = HtmlToMarkdown::convert("<p><a href='https://example.com'>Link</a></p>", $options);

```
