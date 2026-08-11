---
id: fixture_php_image_linked
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
$result = HtmlToMarkdown::convert("<a href=\"https://example.com\"><img src=\"icon.png\" alt=\"Icon\"></a>");

```
