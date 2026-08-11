---
id: fixture_php_blockquote_nested_list_indentation_preserved
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
$result = HtmlToMarkdown::convert("<blockquote><ul><li>item a<ul><li>sub a1</li></ul></li></ul></blockquote>");

```
