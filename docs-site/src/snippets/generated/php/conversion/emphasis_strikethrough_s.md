---
id: fixture_php_emphasis_strikethrough_s
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
$result = HtmlToMarkdown::convert("<p><s>strikethrough</s></p>");

```
