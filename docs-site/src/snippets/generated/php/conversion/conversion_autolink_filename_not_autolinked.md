---
id: fixture_php_conversion_autolink_filename_not_autolinked
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
$result = HtmlToMarkdown::convert("<a href=\"foobar.png\">foobar.png</a>");

```
