---
id: fixture_php_result_tables_without_structure_flag
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
$result = HtmlToMarkdown::convert("<table><tr><th>X</th></tr><tr><td>Y</td></tr></table>");

```
