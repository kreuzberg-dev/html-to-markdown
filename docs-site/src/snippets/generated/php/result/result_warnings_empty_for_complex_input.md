```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
$result = HtmlToMarkdown::convert("<article><h1>Article</h1><p>Paragraph with <strong>bold</strong> and <em>italic</em>.</p><table><tr><th>Col</th></tr><tr><td>Val</td></tr></table><ul><li>Item 1</li><li>Item 2</li></ul></article>");

```
