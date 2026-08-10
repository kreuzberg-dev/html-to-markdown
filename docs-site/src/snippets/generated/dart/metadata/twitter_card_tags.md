```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{}');
  final result = await H2mBridge.convert('<html><head><meta name="twitter:card" content="summary_large_image"><meta name="twitter:site" content="@examplesite"><meta name="twitter:title" content="Twitter Card Title"><meta name="twitter:description" content="Twitter card description."><meta name="twitter:image" content="https://example.com/twitter-image.jpg"></head><body><p>Content</p></body></html>', options: _options);
}

```
