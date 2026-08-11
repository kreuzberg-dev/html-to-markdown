---
id: fixture_php_semantic_mark_highlight
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
$result = HtmlToMarkdown::convert("<p>This is <mark>highlighted text</mark> in a sentence.</p>");

```
