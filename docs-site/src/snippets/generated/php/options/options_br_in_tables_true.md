---
id: fixture_php_options_br_in_tables_true
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
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["brInTables" => true]));
$result = HtmlToMarkdown::convert("<table><tr><th>Header</th></tr><tr><td>Line 1<br>Line 2</td></tr></table>", $options);

```
