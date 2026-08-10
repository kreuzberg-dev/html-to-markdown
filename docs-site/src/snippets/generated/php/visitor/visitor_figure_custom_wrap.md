```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
use HtmlToMarkdown\ConversionOptions;
use HtmlToMarkdown\VisitorHandle;
$visitor = new class {
    public function visit_figure_end(...$args) {
        return ['Custom' => "{output}\n[/FIGURE]\n"];
    }
    public function visit_figure_start(...$args) {
        return ['Custom' => "\n[FIGURE]\n"];
    }
};
$options = \HtmlToMarkdown\ConversionOptions::from_json('{}');
$visitorHandle = \HtmlToMarkdown\VisitorHandle::from_php_object($visitor);
$options = $options->withVisitor($visitorHandle);
$result = HtmlToMarkdown::convert("<section><h2>Gallery</h2><figure><img src=\"photo1.jpg\" alt=\"Photo\"><figcaption>Beautiful sunset</figcaption></figure></section>", $options);

```
