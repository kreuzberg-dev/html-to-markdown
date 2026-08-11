---
id: fixture_php_link_image_inside
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
$result = HtmlToMarkdown::convert("<a href=\"https://example.com\"><img src=\"logo.png\" alt=\"Logo\"></a>");

```
