```php
use HtmlToMarkdown\HtmlToMarkdownApi;
use HtmlToMarkdown\ConversionOptions;

$html = <<<HTML
<table>
    <tr><th>Name</th><th>Age</th></tr>
    <tr><td>Alice</td><td>30</td></tr>
    <tr><td>Bob</td><td>25</td></tr>
</table>
HTML;

// tables are populated only when includeDocumentStructure is enabled.
$options = ConversionOptions::from_json(json_encode(['includeDocumentStructure' => true]));
$result = HtmlToMarkdownApi::convert($html, $options);

foreach ($result->getTables() as $table) {
    foreach ($table->getGrid()->getCells() as $cell) {
        $kind = $cell->isHeader ? 'Header' : 'Cell';
        echo "  {$kind} (r{$cell->row},c{$cell->col}): {$cell->content}\n";
    }
}
```
