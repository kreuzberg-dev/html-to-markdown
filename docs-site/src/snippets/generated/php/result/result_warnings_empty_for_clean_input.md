---
id: fixture_php_result_warnings_empty_for_clean_input
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
$result = HtmlToMarkdown::convert("<h1>Title</h1><p>Clean content with <a href='https://example.com'>a link</a>.</p>");

```
