---
id: fixture_php_conversion_autolink_relative_path_not_autolinked
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
$result = HtmlToMarkdown::convert("<a href=\"/docs/intro.html\">/docs/intro.html</a>");

```
