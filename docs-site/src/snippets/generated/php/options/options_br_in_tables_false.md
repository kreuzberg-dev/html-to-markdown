---
id: fixture_php_options_br_in_tables_false
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
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["brInTables" => false]));
$result = HtmlToMarkdown::convert("<table><tr><th>Col</th></tr><tr><td>A<br>B</td></tr></table>", $options);

```
