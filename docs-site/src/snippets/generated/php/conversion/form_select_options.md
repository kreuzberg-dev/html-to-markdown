```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
use HtmlToMarkdown\ConversionOptions;
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["preprocessing" => ["removeForms" => false]]));
$result = HtmlToMarkdown::convert("<form><label>Color:</label><select><option value=\"red\">Red</option><option value=\"blue\" selected>Blue</option><option value=\"green\">Green</option></select></form>", $options);

```
