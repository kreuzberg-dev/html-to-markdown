---
id: fixture_php_image_with_title
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
$result = HtmlToMarkdown::convert("<img src=\"chart.png\" alt=\"Sales chart\" title=\"Q3 Sales\">");

```
