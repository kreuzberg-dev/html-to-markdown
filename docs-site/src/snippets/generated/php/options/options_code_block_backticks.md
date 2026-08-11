---
id: fixture_php_options_code_block_backticks
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
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["codeBlockStyle" => "Backticks"]));
$result = HtmlToMarkdown::convert("<pre><code class=\"language-js\">console.log('hi');</code></pre>", $options);

```
