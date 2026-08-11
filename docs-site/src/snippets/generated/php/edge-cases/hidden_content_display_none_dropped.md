---
id: fixture_php_hidden_content_display_none_dropped
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
$result = HtmlToMarkdown::convert("<p>visible</p><div style=\"display:none\">secret hidden text</div><p>also visible</p>");

```
