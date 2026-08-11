---
id: fixture_php_xss_script_tag_stripped
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
$result = HtmlToMarkdown::convert("<p>Safe content.</p><script>alert('xss')</script><p>More safe content.</p>");

```
