---
id: fixture_php_options_preserve_tags_iframe
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
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["preserveTags" => ["iframe"]]));
$result = HtmlToMarkdown::convert("<p>Before</p><iframe src='video.html' width='560'></iframe><p>After</p>", $options);

```
