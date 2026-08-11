---
id: fixture_php_malformed_bogus_comment_triple_dash
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
$result = HtmlToMarkdown::convert("<h1>One</h1>\n<!-- /// --->\n<p>Two</p>");

```
