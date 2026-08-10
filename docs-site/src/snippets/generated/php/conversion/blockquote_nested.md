```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
$result = HtmlToMarkdown::convert("<blockquote><p>Outer quote.</p><blockquote><p>Inner quote.</p></blockquote></blockquote>");

```
