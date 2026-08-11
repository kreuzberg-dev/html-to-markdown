---
id: fixture_php_paragraph_with_line_breaks
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
$result = HtmlToMarkdown::convert("<p>Line one.<br>Line two.<br>Line three.</p>");

```
