```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
$result = HtmlToMarkdown::convert("<html><head><script>alert('xss')</script></head><body><script>document.write('hello')</script></body></html>");

```
