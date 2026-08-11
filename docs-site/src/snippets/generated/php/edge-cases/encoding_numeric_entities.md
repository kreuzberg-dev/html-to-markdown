---
id: fixture_php_encoding_numeric_entities
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
$result = HtmlToMarkdown::convert("<p>Copyright: &#169; Trade: &#174; Euro: &#8364; Hex: &#x00A9;</p>");

```
