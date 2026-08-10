```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{}');
  final result = await H2mBridge.convert('<html><head><title>Fallback Title</title><meta property="og:title" content="OG Title"><meta property="og:description" content="OG description text."><meta property="og:image" content="https://example.com/image.jpg"></head><body><p>Content</p></body></html>', options: _options);
}

```
