---
id: fixture_php_hidden_content_aria_hidden_still_rendered
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
$result = HtmlToMarkdown::convert("<p>visible</p><div aria-hidden=\"true\">still shown</div><p>also visible</p>");

```
