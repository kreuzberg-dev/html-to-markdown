---
id: fixture_php_html_comments_only
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
$result = HtmlToMarkdown::convert("<!-- This is a comment --><!-- Another comment -->");

```
