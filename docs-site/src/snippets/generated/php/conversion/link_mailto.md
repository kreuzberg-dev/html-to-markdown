---
id: fixture_php_link_mailto
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
$result = HtmlToMarkdown::convert("<a href=\"mailto:user@example.com\">Email us</a>");

```
