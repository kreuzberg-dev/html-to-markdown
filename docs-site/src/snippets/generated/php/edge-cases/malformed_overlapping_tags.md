---
id: fixture_php_malformed_overlapping_tags
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
$result = HtmlToMarkdown::convert("<p><b><i>bold and italic</b></i></p>");

```
