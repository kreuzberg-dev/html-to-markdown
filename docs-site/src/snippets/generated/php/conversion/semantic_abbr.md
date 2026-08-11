---
id: fixture_php_semantic_abbr
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
$result = HtmlToMarkdown::convert("<p>The <abbr title=\"World Wide Web\">WWW</abbr> is global.</p>");

```
