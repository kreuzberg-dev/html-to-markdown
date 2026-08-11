---
id: fixture_php_semantic_sub_superscript
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
$result = HtmlToMarkdown::convert("<p>H<sub>2</sub>O and E=mc<sup>2</sup></p>");

```
