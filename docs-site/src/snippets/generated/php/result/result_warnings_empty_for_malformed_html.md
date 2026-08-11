---
id: fixture_php_result_warnings_empty_for_malformed_html
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
$result = HtmlToMarkdown::convert("<p>Unclosed paragraph<div>Mixed nesting</p></div>");

```
