---
id: fixture_php_link_with_bold_text
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
$result = HtmlToMarkdown::convert("<a href=\"https://example.com\"><strong>Bold link</strong></a>");

```
