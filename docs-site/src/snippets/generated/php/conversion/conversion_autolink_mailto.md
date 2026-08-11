---
id: fixture_php_conversion_autolink_mailto
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
$result = HtmlToMarkdown::convert("<a href=\"mailto:a@b.com\">a@b.com</a>");

```
