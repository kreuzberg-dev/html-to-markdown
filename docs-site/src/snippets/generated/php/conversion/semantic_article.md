---
id: fixture_php_semantic_article
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
$result = HtmlToMarkdown::convert("<article><h2>Article Title</h2><p>Article body.</p></article>");

```
