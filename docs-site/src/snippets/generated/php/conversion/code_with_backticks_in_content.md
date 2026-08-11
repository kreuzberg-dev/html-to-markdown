---
id: fixture_php_code_with_backticks_in_content
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
$result = HtmlToMarkdown::convert("<p>Use <code>`backtick` here</code> carefully.</p>");

```
