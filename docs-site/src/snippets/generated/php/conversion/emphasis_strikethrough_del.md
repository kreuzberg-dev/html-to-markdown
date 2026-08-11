---
id: fixture_php_emphasis_strikethrough_del
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
$result = HtmlToMarkdown::convert("<p><del>deleted text</del></p>");

```
