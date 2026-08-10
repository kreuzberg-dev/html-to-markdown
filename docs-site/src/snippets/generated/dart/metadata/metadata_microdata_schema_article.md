```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"extract_metadata":true}');
  final result = await H2mBridge.convert('<html><head><title>Article</title></head><body><article itemscope itemtype="https://schema.org/Article"><h1 itemprop="headline">Breaking News Today</h1><span itemprop="author">Jane Reporter</span><span itemprop="datePublished">2024-04-22</span><div itemprop="articleBody"><p>The article content goes here with important information about the breaking news story.</p></div></article></body></html>', options: _options);
}

```
