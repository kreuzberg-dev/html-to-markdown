---
id: fixture_php_paragraph_nested_divs
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
$result = HtmlToMarkdown::convert("<div><div><p>Nested text</p></div></div>");

```
