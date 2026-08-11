---
id: fixture_php_code_block
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
$result = HtmlToMarkdown::convert("<pre><code class=\"language-python\">print('hello')</code></pre>");

```
