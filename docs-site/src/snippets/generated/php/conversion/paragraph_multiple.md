---
id: fixture_php_paragraph_multiple
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
$result = HtmlToMarkdown::convert("<p>First paragraph.</p><p>Second paragraph.</p>");

```
