---
id: fixture_php_script_tags_only
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
$result = HtmlToMarkdown::convert("<html><head><script>alert('xss')</script></head><body><script>document.write('hello')</script></body></html>");

```
