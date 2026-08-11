---
id: fixture_php_hidden_content_template_element_dropped
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
$result = HtmlToMarkdown::convert("<p>visible</p><template><p>secret template text</p></template><p>also visible</p>");

```
