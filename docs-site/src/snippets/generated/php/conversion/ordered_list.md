---
id: fixture_php_ordered_list
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
$result = HtmlToMarkdown::convert("<ol><li>First</li><li>Second</li><li>Third</li></ol>");

```
