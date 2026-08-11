---
id: fixture_php_encoding_cjk_characters
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
$result = HtmlToMarkdown::convert("<p>中文内容</p><p>日本語テキスト</p><p>한국어 텍스트</p>");

```
