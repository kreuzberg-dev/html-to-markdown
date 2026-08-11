---
id: fixture_php_image_simple
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
$result = HtmlToMarkdown::convert("<img src=\"photo.jpg\" alt=\"A photo\">");

```
