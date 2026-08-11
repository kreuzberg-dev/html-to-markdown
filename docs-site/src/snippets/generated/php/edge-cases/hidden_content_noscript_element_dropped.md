---
id: fixture_php_hidden_content_noscript_element_dropped
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
$result = HtmlToMarkdown::convert("<p>visible</p><noscript><p>secret noscript text</p></noscript><p>also visible</p>");

```
