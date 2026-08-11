---
id: fixture_php_line_break_hr_tag
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
$result = HtmlToMarkdown::convert("<p>Before rule.</p><hr><p>After rule.</p>");

```
