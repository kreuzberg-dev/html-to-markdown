```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
use HtmlToMarkdown\ConversionOptions;
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["excludeSelectors" => [".nav", "footer"]]));
$result = HtmlToMarkdown::convert("<body><nav class=\"nav\">Menu</nav><p>Content</p><footer>Footer</footer></body>", $options);

```
