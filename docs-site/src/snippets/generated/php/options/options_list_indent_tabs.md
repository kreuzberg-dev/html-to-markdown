---
id: fixture_php_options_list_indent_tabs
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
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["listIndentType" => "Tabs"]));
$result = HtmlToMarkdown::convert("<ul><li>Parent<ul><li>Child</li></ul></li></ul>", $options);

```
