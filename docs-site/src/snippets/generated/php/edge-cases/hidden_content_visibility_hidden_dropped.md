---
id: fixture_php_hidden_content_visibility_hidden_dropped
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
$result = HtmlToMarkdown::convert("<p>visible</p><span style=\"visibility:hidden\">secret hidden span</span><p>also visible</p>");

```
