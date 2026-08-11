---
id: fixture_php_options_max_depth_truncates
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
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["maxDepth" => 3]));
$result = HtmlToMarkdown::convert("<div><p>Shallow</p><div><div><div><p>Too deep</p></div></div></div></div>", $options);

```
