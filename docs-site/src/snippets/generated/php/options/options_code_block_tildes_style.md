---
id: fixture_php_options_code_block_tildes_style
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
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["codeBlockStyle" => "Tildes"]));
$result = HtmlToMarkdown::convert("<pre><code>some code</code></pre>", $options);

```
