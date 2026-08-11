---
id: fixture_php_options_keep_inline_images_in_paragraph
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
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["keepInlineImagesIn" => ["p"]]));
$result = HtmlToMarkdown::convert("<p>Text <img src='icon.png' alt='icon'> more text</p>", $options);

```
