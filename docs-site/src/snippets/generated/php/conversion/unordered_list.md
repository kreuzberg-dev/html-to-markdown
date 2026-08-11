---
id: fixture_php_unordered_list
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
$result = HtmlToMarkdown::convert("<ul><li>Item 1</li><li>Item 2</li><li>Item 3</li></ul>");

```
