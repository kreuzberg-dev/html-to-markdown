---
id: fixture_php_conversion_autolink_https_url
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
$result = HtmlToMarkdown::convert("<a href=\"https://example.com\">https://example.com</a>");

```
